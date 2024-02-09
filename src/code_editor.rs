use std::sync::atomic::{AtomicBool, Ordering};

use js_sys::Object;
use monaco::{
    api::CodeEditorOptions,
    sys::editor::{
        self, ICodeEditor, IEditorHoverOptions, IEditorOptionsTabCompletion, IStandaloneCodeEditor,
        IStandaloneEditorConstructionOptions,
    },
    sys::{KeyCode, KeyMod},
    yew::{CodeEditor, CodeEditorLink},
};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

use crate::{
    monaco_ram::{register_ram, LANG_ID, THEME},
    monaco_tweaks::setup_breakpoints,
    store::{dispatch, Store},
    utils::{comment_code, download_code},
};

type JsCallback = Closure<dyn Fn()>;

pub const DEFAULT_CODE: &str = r"
read 1
write 1
read 2
write 2
halt
";

pub fn get_editor_options(read_only: bool) -> IStandaloneEditorConstructionOptions {
    let options = CodeEditorOptions::default()
        .with_language(LANG_ID.to_owned())
        .with_theme(THEME.to_owned())
        .with_automatic_layout(true)
        .to_sys_options();

    let hover_options = Object::new().unchecked_into::<IEditorHoverOptions>(); // Doesn't work
    hover_options.set_enabled(Some(true));
    hover_options.set_delay(Some(300.));

    options.set_font_size(Some(16.0));
    options.set_tab_completion(Some(IEditorOptionsTabCompletion::On));
    options.set_read_only(Some(read_only));
    options.set_glyph_margin(Some(true));
    options.set_line_numbers(Some(editor::LineNumbersType::Relative));
    options.set_hover(Some(&hover_options)); // Doesn't work
    options
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub run_code: Callback<()>,
    pub read_only: bool,
    pub line: usize,
}

pub enum Msg {
    EditorCreated(CodeEditorLink),
    DownloadCode,
    CommentCode,
}

pub struct CustomEditor {
    editor_ref: NodeRef,
}

/// # Panics
/// Panics if rendered before hydration.
impl Component for CustomEditor {
    type Message = Msg;
    type Properties = Props;

    fn view(&self, ctx: &Context<Self>) -> Html {
        let read_only = ctx.props().read_only;

        let on_editor_created = ctx.link().callback(Msg::EditorCreated);
        let model = Some(dispatch().get().get_model().clone());

        html! {
          <div id="container" class="editor-container" ref={&self.editor_ref}>
              <CodeEditor
                classes={"editor"}
                options={get_editor_options(read_only)}
                {model}
                {on_editor_created}
              />
          </div>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        log::debug!("Editor Component Created");
        monaco::workers::ensure_environment_set();
        register_ram();

        let editor_ref: NodeRef = Default::default();

        let text_model = dispatch().get().get_model().clone();
        let text_model_saver = text_model.on_did_change_content(move |_| {
            dispatch().reduce_mut(move |s: &mut Store| s.change_model());
        });
        std::mem::forget(text_model_saver);

        let code_in_url = gloo::utils::window()
            .location()
            .search()
            .expect("No search in URL!")
            .replace('?', "")
            .split('&')
            .find(|x| x.starts_with("code="))
            .map(|x| x.replace("code=", ""));

        if let Some(code_in_url) = code_in_url {
            if let Ok(code) = urlencoding::decode(&code_in_url) {
                text_model.set_value(&code);
            }
        }
        Self { editor_ref }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let read_only = ctx.props().read_only;
        let editor = dispatch().get().editor.clone();
        if read_only != old_props.read_only {
            editor.with_editor(|editor| {
                let ieditor: &ICodeEditor = editor.as_ref();
                ieditor.update_options(&get_editor_options(read_only));
            });
        };
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let text_model = dispatch().get().get_model().clone();
        let run_code = &ctx.props().run_code;

        match msg {
            Msg::DownloadCode => {
                if let Err(err) = download_code(&text_model.get_value()) {
                    gloo::console::error!("Failed to download code: ", err);
                }
            }
            Msg::CommentCode => {
                let editor = dispatch().get().editor.clone();
                editor.with_editor(|editor| comment_code(editor, &text_model));
            }
            Msg::EditorCreated(editor_link) => {
                // highlight_error(&editor_link, "some error", 1, 1);
                static EDITOR_WAS_CREATED: AtomicBool = AtomicBool::new(false);

                if EDITOR_WAS_CREATED.swap(true, Ordering::Relaxed) {
                    panic!("Editor was created twice!");
                }

                log::info!("Editor Created");
                dispatch().reduce_mut(|s: &mut Store| s.editor = editor_link.clone());

                let run_code = run_code.clone();
                let code_runner = JsCallback::new(move || run_code.emit(()));
                let link = ctx.link().clone();
                let downloader = JsCallback::new(move || link.send_message(Msg::DownloadCode));
                let link = ctx.link().clone();
                let commenter = JsCallback::new(move || link.send_message(Msg::CommentCode));

                editor_link.with_editor(|editor| {
                    let run_code = KeyCode::Enter.to_value() | (KeyMod::ctrl_cmd() as u32);
                    let save_code = KeyCode::KeyS.to_value() | (KeyMod::ctrl_cmd() as u32);
                    let comment_code = KeyCode::UsSlash.to_value() | (KeyMod::ctrl_cmd() as u32);
                    let code_runner = code_runner.as_ref().unchecked_ref();
                    let downloader = downloader.as_ref().unchecked_ref();
                    let commenter = commenter.as_ref().unchecked_ref();

                    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
                    raw_editor.add_command(run_code.into(), code_runner, None);
                    raw_editor.add_command(save_code.into(), downloader, None);
                    raw_editor.add_command(comment_code.into(), commenter, None);

                    std::mem::forget(setup_breakpoints(editor));
                });

                // It iis okay to forget these callbacks because editor will not be created twice
                // so this is not a real memory leak - it will behave like a normal value, and will
                // be freed when the editor is dropped, becouse it will be the end of the program.
                // Editor will not be dropped before the end of the program, so it is okay.
                code_runner.forget();
                downloader.forget();
                commenter.forget();
            }
        }
        false
    }
}
