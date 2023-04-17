use std::borrow::BorrowMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::code_editor::CustomEditor;
use crate::code_runner::CodeRunner;
use crate::header::Header;
use crate::memory::Memory;
use crate::utils::comment_selected_code;
use crate::utils::download_code;
use crate::utils::get_from_local_storage;
use crate::utils::save_to_local_storage;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::html::Scope;
use yew::prelude::*;
type JsCallback = Closure<dyn Fn()>;
use crate::code_runner::Msg as CodeRunnerMsg;
use monaco::sys::KeyCode;
use monaco::sys::KeyMod;
use monaco::{api::TextModel, sys::editor::IStandaloneCodeEditor, yew::CodeEditorLink};
use ramemu::registers::Registers;

pub struct App {
  scope: Scope<Self>,
  code_runner_scope: Option<Scope<CodeRunner>>,
  // stdin: String,
  editor: Option<CodeEditorLink>,
  editor_ref: NodeRef,
  text_model: TextModel,
  memory: Registers<i64>,
  default_code: String,
}

pub enum Msg {
  EditorCreated(CodeEditorLink),
  SetRunnerScope(Scope<CodeRunner>),
  SetMemory(Registers<i64>),
  DownloadCode,
  SaveCodeToLocalStorage,
  CommentCode,
  RunCode,
}

const INITIAL_CODE: &str = r#"
read 1
write 1
read 2
write 2
halt
"#;

static EDITOR_WAS_CREATED: AtomicBool = AtomicBool::new(false);

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let editor_ref: NodeRef = Default::default();
    let code = get_from_local_storage("code").unwrap_or_else(|| INITIAL_CODE.to_string());

    Self {
      scope: ctx.link().clone(),
      // stdin: Default::default(),
      memory: Default::default(),
      editor: None,
      code_runner_scope: None,
      editor_ref,
      text_model: TextModel::create(code.as_str(), Some("ram"), None)
        .expect("Failed to create text model"),
      default_code: code,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::EditorCreated(editor_link) => {
        if EDITOR_WAS_CREATED.fetch_or(true, Ordering::SeqCst) {
          panic!("Editor was created twice!");
        }

        log::info!("Editor Created");
        self.editor = Some(editor_link.clone());
        let link1 = ctx.link().clone();
        let link2 = ctx.link().clone();
        let link3 = ctx.link().clone();
        let link4 = ctx.link().clone();

        let code_runner = JsCallback::new(move || link1.send_message(Msg::DownloadCode));
        let downloader = JsCallback::new(move || link2.send_message(Msg::CommentCode));
        let commenter = JsCallback::new(move || link3.send_message(Msg::CommentCode));
        let code_saver = JsCallback::new(move || {
          let link = link4.clone();
          let code_saver_core =
            JsCallback::new(move || link.send_message(Msg::SaveCodeToLocalStorage));
          log::info!("Saving!");
          window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
              code_saver_core.as_ref().unchecked_ref(),
              50,
            )
            .unwrap();
          code_saver_core.forget();
        });

        self
          .editor_ref
          .get()
          .unwrap()
          .add_event_listener_with_callback("keydown", code_saver.as_ref().unchecked_ref())
          .expect("Failed to add event listener");

        editor_link.with_editor(|editor| {
          let run_code = KeyCode::Enter.to_value() | (KeyMod::ctrl_cmd() as u32);
          let save_code = KeyCode::KeyS.to_value() | (KeyMod::ctrl_cmd() as u32);
          let comment_code = KeyCode::UsSlash.to_value() | (KeyMod::ctrl_cmd() as u32);
          let code_runner = code_runner.as_ref().unchecked_ref();
          let downloader = downloader.as_ref().unchecked_ref();
          let commenter = commenter.as_ref().unchecked_ref();
          ctx.link().send_message(Msg::SaveCodeToLocalStorage);

          let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
          raw_editor.add_command(run_code.into(), code_runner, None);
          raw_editor.add_command(save_code.into(), downloader, None);
          raw_editor.add_command(comment_code.into(), commenter, None);

          save_to_local_storage("code", &self.text_model.get_value());
        });

        code_runner.forget();
        downloader.forget();
        commenter.forget();
        code_saver.forget();
      }
      Msg::DownloadCode => {
        download_code(&self.text_model.get_value()).expect("Failed to download code")
      }
      Msg::SaveCodeToLocalStorage => save_to_local_storage("code", &self.text_model.get_value()),
      Msg::CommentCode => {
        if let Some(editor) = &self.editor {
          editor.with_editor(|editor| comment_selected_code(editor, &self.text_model));
        }
      }
      Msg::SetRunnerScope(scope) => self.code_runner_scope = Some(scope),
      Msg::SetMemory(memory) => {
        self.memory = memory;
        return true;
      }
      Msg::RunCode => {
        if let Some(s) = &self.code_runner_scope {
          s.send_message(CodeRunnerMsg::RunCode);
        }
      }
    };
    false
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let on_editor_created = self.scope.callback(Msg::EditorCreated);
    let on_start = self.scope.callback(|_| Msg::RunCode);
    let on_pause = self.scope.batch_callback(|_| None); // TODO:
    let on_stop = self.scope.batch_callback(|_| None); // TODO:
    let on_debug = self.scope.batch_callback(|_| None); // TODO:

    html! {
      <main id="ram-web">
        <Header {on_start} {on_stop} {on_pause} {on_debug} />

        <div class="interface">
          <div class="editor-registers">
              <div id="container" class="editor-container" ref={self.editor_ref.clone()}>
                <CustomEditor
                  value={self.default_code.clone()}
                  {on_editor_created}
                  text_model={self.text_model.clone()}
                />
              </div>
              <Memory entries={self.memory.clone()} />
          </div>
        </div>

        <CodeRunner
          code={self.text_model.get_value().clone()}
          set_memory={self.scope.callback(Msg::SetMemory)}
          set_scope={self.scope.callback(Msg::SetRunnerScope)}
        />

      </main>
    }
  }

  // TODO: add loader with portal or whatever
  // <div id="loader">
  //     <div class="orbe" style="--index: 0"></div>
  //     <div class="orbe" style="--index: 1"></div>
  //     <div class="orbe" style="--index: 2"></div>
  //     <div class="orbe" style="--index: 3"></div>
  //     <div class="orbe" style="--index: 4"></div>
  // </div>
}
