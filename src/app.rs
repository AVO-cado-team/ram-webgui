use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::code_editor::CustomEditor;
use crate::code_runner::CodeRunner;
use crate::header::Header;
use crate::memory::Memory;
use crate::utils::comment_code;
use crate::utils::download_code;
use crate::utils::get_from_local_storage;
use crate::utils::save_to_local_storage;
  // // Move the cursor or clear the selection after editing
  // let ieditor: &IEditor = editor.as_ref();
  // if range.start_line_number() == range.end_line_number() {
  //   // Single line comment
  //   let column = ieditor.get_position().unwrap().column();
  //   let column = column + if new_text.starts_with('#') { 1.0 } else { -1.0 };
  //   let column = column.max(1.0);
  //   let new_position = Position::new(range.start_line_number(), column);
  //   ieditor.set_position(<Position as AsRef<Position>>::as_ref(&new_position).unchecked_ref())
  //   // ieditor.set_position(new_position.as_ref().unchecked_ref());
  // }
  // // No need to clear the selection if it's a multiline comment

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
  code_runner_scope: Option<Scope<CodeRunner>>,
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
  SaveCode,
  CommentCode,
  RunCode,
}

const DEFAULT_CODE: &str = r#"
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

  fn create(_ctx: &Context<Self>) -> Self {
    let editor_ref: NodeRef = Default::default();
    let code = get_from_local_storage("code").unwrap_or_else(|| DEFAULT_CODE.to_string());
    let text_model =
      TextModel::create(code.as_str(), Some("ram"), None).expect("Failed to create text model");

    {
      let text_model = text_model.clone();
      let on_before_unload = Closure::wrap(Box::new(move |_| {
        save_to_local_storage("code", &text_model.get_value());
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
    }

    Self {
      memory: Default::default(),
      editor: None,
      code_runner_scope: None,
      editor_ref,
      text_model,
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
          ctx.link().send_message(Msg::SaveCode);

          let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
          raw_editor.add_command(run_code.into(), code_runner, None);
          raw_editor.add_command(save_code.into(), downloader, None);
          raw_editor.add_command(comment_code.into(), commenter, None);

          save_to_local_storage("code", &self.text_model.get_value());
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
      Msg::DownloadCode => {
        download_code(&self.text_model.get_value()).expect("Failed to download code")
      }
      Msg::SaveCode => {
        log::info!("Saving!");
        save_to_local_storage("code", &self.text_model.get_value());
        return true;
      }
      Msg::CommentCode => {
        if let Some(editor) = &self.editor {
          editor.with_editor(|editor| comment_code(editor, &self.text_model));
        }
        return true;
      }
      Msg::SetRunnerScope(scope) => self.code_runner_scope = Some(scope),
      Msg::SetMemory(memory) => {
        self.memory = memory;
        return true;
      }
      Msg::RunCode => {
        if let Some(s) = &self.code_runner_scope {
          s.send_message(CodeRunnerMsg::RunCode(self.text_model.get_value()));
        }
      }
    };
    false
  }

  fn destroy(&mut self, _ctx: &Context<Self>) {
    let new_code = self.text_model.get_value() + " MARKER";
    save_to_local_storage("code", &new_code);
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let on_editor_created = ctx.link().callback(Msg::EditorCreated);
    let on_start = ctx.link().callback(|_| Msg::RunCode);
    let on_pause = ctx.link().batch_callback(|_| None); // TODO:
    let on_stop = ctx.link().batch_callback(|_| None); // TODO:
    let on_debug = ctx.link().batch_callback(|_| None); // TODO:

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
          set_memory={ctx.link().callback(Msg::SetMemory)}
          set_scope={ctx.link().callback(Msg::SetRunnerScope)}
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
