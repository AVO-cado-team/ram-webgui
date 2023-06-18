use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use web_sys::window;

const DEFAULT_CODE: &str = r#"
read 1
write 1
read 2
write 2
halt
"#;

use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::ICodeEditor,
    sys::editor::IStandaloneCodeEditor,
    sys::editor::{IEditorOptionsTabCompletion, IStandaloneEditorConstructionOptions},
    sys::KeyCode,
    sys::KeyMod,
    yew::{CodeEditor, CodeEditorLink},
};
use yew::prelude::*;

use crate::monaco_ram::register_ram;
use crate::monaco_ram::{LANG_ID, THEME};
use crate::utils::comment_code;
use crate::utils::download_code;
use crate::utils::get_from_local_storage;
use crate::utils::save_to_local_storage;

type JsCallback = Closure<dyn Fn()>;

static EDITOR_WAS_CREATED: AtomicBool = AtomicBool::new(false);

pub fn get_editor_options(read_only: bool) -> IStandaloneEditorConstructionOptions {
    let options = CodeEditorOptions::default()
        .with_language(LANG_ID.to_owned())
        .with_theme(THEME.to_owned())
        .with_automatic_layout(true)
        .to_sys_options();

    options.set_font_size(Some(16.0));
    options.set_tab_completion(Some(IEditorOptionsTabCompletion::On));
    options.set_read_only(Some(read_only));

    options
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub run_code: Callback<()>,
    pub read_only: bool,
    pub set_text_model: Callback<TextModel>,
}

pub enum Msg {
    EditorCreated(CodeEditorLink),
    SaveCode,
    DownloadCode,
    CommentCode,
}

pub struct CustomEditor {
    editor: Option<CodeEditorLink>,
    editor_ref: NodeRef,
    text_model: TextModel,
}

//  WARN: Should not be rendered before hydration.
//        Will panic due to calls to web apis.

impl Component for CustomEditor {
    type Message = Msg;
    type Properties = Props;

    fn view(&self, ctx: &Context<Self>) -> Html {
        let read_only = ctx.props().read_only;

        let on_editor_created = ctx.link().callback(Msg::EditorCreated);

        html! {
          <div id="container" class="editor-container" ref={self.editor_ref.clone()}>
              <CodeEditor
                classes={"editor"}
                options={get_editor_options(read_only)}
                model={self.text_model.clone()}
                {on_editor_created}
              />
          </div>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("Editor Created");
        monaco::workers::ensure_environment_set();
        register_ram();

        let code = get_from_local_storage("code").unwrap_or_else(|| DEFAULT_CODE.to_string());
        let text_model = TextModel::create(code.as_str(), Some("ram"), None)
            .expect("Failed to create text model");
        let text_model_clone = text_model.clone();

        ctx.props().set_text_model.emit(text_model.clone());

        let editor_ref: NodeRef = Default::default();
        let editor = None;

        let on_before_unload = Closure::wrap(Box::new(move |_| {
            save_to_local_storage("code", &text_model_clone.get_value());
        }) as Box<dyn FnMut(web_sys::Event)>);

        if let Some(window) = window() {
            window
                .add_event_listener_with_callback(
                    "beforeunload",
                    on_before_unload.as_ref().unchecked_ref(),
                )
                .expect("Failed to add beforeunload event listener");
        }
        on_before_unload.forget();

        Self {
            editor,
            editor_ref,
            text_model,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let read_only = ctx.props().read_only;

        if read_only != old_props.read_only {
            if let Some(editor) = &self.editor {
                editor.with_editor(|editor| {
                    let ieditor: &ICodeEditor = editor.as_ref();
                    ieditor.update_options(&get_editor_options(read_only));
                });
            }
        }

        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let text_model = &self.text_model;
        let run_code = &ctx.props().run_code;

        match msg {
            Msg::DownloadCode => {
                download_code(&text_model.get_value()).expect("Failed to download code")
            }
            Msg::CommentCode => {
                if let Some(editor) = &self.editor {
                    editor.with_editor(|editor| comment_code(editor, text_model));
                }
            }
            Msg::SaveCode => {
                log::info!("Saving!");
                save_to_local_storage("code", &text_model.get_value());
            }
            Msg::EditorCreated(editor_link) => {
                if EDITOR_WAS_CREATED.fetch_or(true, Ordering::SeqCst) {
                    panic!("Editor was created twice!");
                }

                log::info!("Editor Created");
                self.editor = Some(editor_link.clone());
                let link1 = ctx.link().clone();
                let link2 = ctx.link().clone();
                let link3 = ctx.link().clone();

                let run_code = run_code.clone();
                let code_runner = JsCallback::new(move || run_code.emit(()));
                let downloader = JsCallback::new(move || link1.send_message(Msg::DownloadCode));
                let commenter = JsCallback::new(move || link2.send_message(Msg::CommentCode));
                let code_saver = JsCallback::new(move || {
                    let link = link3.clone();
                    let code_saver_core = JsCallback::new(move || link.send_message(Msg::SaveCode));
                    window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            code_saver_core.as_ref().unchecked_ref(),
                            50,
                        )
                        .unwrap();
                    code_saver_core.forget();
                });

                self.editor_ref
                    .get()
                    .unwrap()
                    .add_event_listener_with_callback(
                        "keydown",
                        code_saver.as_ref().unchecked_ref(),
                    )
                    .expect("Failed to add event listener");

                editor_link.with_editor(|editor| {
                    let run_code = KeyCode::Enter.to_value() | (KeyMod::ctrl_cmd() as u32);
                    let save_code = KeyCode::KeyS.to_value() | (KeyMod::ctrl_cmd() as u32);
                    let comment_code = KeyCode::UsSlash.to_value() | (KeyMod::ctrl_cmd() as u32);
                    let code_runner = code_runner.as_ref().unchecked_ref();
                    let downloader = downloader.as_ref().unchecked_ref();
                    let commenter = commenter.as_ref().unchecked_ref();
                    ctx.link().send_message(Msg::SaveCode);

                    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
                    raw_editor.add_command(run_code.into(), code_runner, None);
                    raw_editor.add_command(save_code.into(), downloader, None);
                    raw_editor.add_command(comment_code.into(), commenter, None);
                });

                // It iis okay to forget these callbacks because editor will not be created twice
                // so this is not a real memory leak - it will behave like a normal value, and will
                // be freed when the editor is dropped, becouse it will be the end of the program.
                // Editor will not be dropped before the end of the program, so it is okay.
                code_runner.forget();
                downloader.forget();
                commenter.forget();
                code_saver.forget();
            }
        }
        false
    }
}
