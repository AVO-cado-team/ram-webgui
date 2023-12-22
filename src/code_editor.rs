use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use gloo::events::EventListener;
use gloo::storage::LocalStorage;
use gloo::storage::Storage;
use monaco::api::DisposableClosure;
use monaco::sys::editor::IModelContentChangedEvent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const DEFAULT_CODE: &str = r"
read 1
write 1
read 2
write 2
halt
";

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

type JsCallback = Closure<dyn Fn()>;

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
    pub line: usize,
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
    _on_before_unload: EventListener,
    _text_model_saver: DisposableClosure<dyn FnMut(IModelContentChangedEvent)>,
}

/// # Panics
/// Panics if rendered before hydration.
impl Component for CustomEditor {
    type Message = Msg;
    type Properties = Props;

    fn view(&self, ctx: &Context<Self>) -> Html {
        let read_only = ctx.props().read_only;

        let on_editor_created = ctx.link().callback(Msg::EditorCreated);

        // todo(ic-it): use `ctx.props().line()` to highlight current line in debug mode
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
        log::info!("Editor Component Created");
        monaco::workers::ensure_environment_set();
        register_ram();

        let code: String = LocalStorage::get("code").unwrap_or_else(|_| DEFAULT_CODE.to_string());
        let text_model = TextModel::create(code.as_str(), Some("ram"), None)
            .expect("Failed to create text model");

        let link = ctx.link().clone();
        let text_model_saver =
            text_model.on_did_change_content(move |_| link.send_message(Msg::SaveCode));

        ctx.props().set_text_model.emit(text_model.clone());

        let editor_ref: NodeRef = Default::default();
        let editor = None;

        let text_model_clone = text_model.clone();
        let on_before_unload =
            EventListener::new(&gloo::utils::window(), "beforeunload", move |_| {
                if let Err(err) = LocalStorage::set("code", text_model_clone.get_value()) {
                    log::error!("Failed to save code: {err:?}");
                }
            });

        Self {
            editor,
            editor_ref,
            text_model,
            _on_before_unload: on_before_unload,
            _text_model_saver: text_model_saver,
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
                if let Err(err) = download_code(&text_model.get_value()) {
                    gloo::console::error!("Failed to download code: ", err);
                }
            }
            Msg::CommentCode => {
                if let Some(editor) = &self.editor {
                    editor.with_editor(|editor| comment_code(editor, text_model));
                }
            }
            Msg::SaveCode => {
                log::info!("Saving!");
                if let Err(err) = LocalStorage::set("code", text_model.get_value()) {
                    log::error!("Failed to save code: {err}");
                }
            }
            Msg::EditorCreated(editor_link) => {
                static EDITOR_WAS_CREATED: AtomicBool = AtomicBool::new(false);

                if EDITOR_WAS_CREATED.swap(true, Ordering::Relaxed) {
                    panic!("Editor was created twice!");
                }

                log::info!("Editor Created");
                self.editor = Some(editor_link.clone());

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
            }
        }
        false
    }
}
