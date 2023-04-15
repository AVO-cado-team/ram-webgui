use crate::code_editor::CustomEditor;
use crate::header::Header;
use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use crate::io::input::InputComponent;
use crate::io::output::OutputComponent;
use crate::io::output::OutputComponentErrors;
use crate::memory::Memory;
use crate::utils::comment_selected_code;
use crate::utils::download_code;

use std::rc::Rc;

use ramemu::ram::RamState;
use ramemu::registers::Registers;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::html::Scope;
use yew::prelude::*;
type MonCallbak = Closure<dyn Fn()>;

use monaco::sys::KeyCode;
use monaco::sys::KeyMod;
use monaco::{api::TextModel, sys::editor::IStandaloneCodeEditor, yew::CodeEditorLink};

use ramemu::program::Program;
use ramemu::ram::Ram;

const INITIAL_CODE: &str = r#"
read 1
write 1
read 2
write 2
halt
"#;

const INITIAL_STDIN: &str = r#" 3 4 "#;

const RUN_CODE_AT_START: bool = false;

pub struct App {
  link: Scope<Self>,
  text_model: TextModel,
  code: String,
  // stdin: String,
  stdout: String,
  editor: Option<CodeEditorLink>,
  code_runner: Rc<Closure<dyn Fn()>>,
  code_saver: Rc<Closure<dyn Fn()>>,
  commenter: Rc<Closure<dyn Fn()>>,
  memory: Registers<i64>,
  error: Option<OutputComponentErrors>,
  reader: CustomReader,
  writer: CustomWriter,
}

pub enum Msg {
  EditorCreated(CodeEditorLink),
  RunCode,
  SaveCode,
  CommentCode,
  WriterWrote(String),
  InputChanged(String),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let link1 = ctx.link().clone();
    let link2 = ctx.link().clone();
    let link3 = ctx.link().clone();
    Self {
      link: ctx.link().clone(),
      text_model: TextModel::create(INITIAL_CODE, Some("ram"), None).unwrap(),
      code: String::from(INITIAL_CODE),
      // stdin: Default::default(),
      stdout: Default::default(),
      memory: Default::default(),
      error: None,
      editor: None,
      reader: CustomReader::new(INITIAL_STDIN.to_string()),
      writer: CustomWriter::new(ctx.link().callback(Msg::WriterWrote)),

      code_runner: Rc::new(MonCallbak::new(move || link1.send_message(Msg::RunCode))),
      code_saver: Rc::new(MonCallbak::new(move || link2.send_message(Msg::SaveCode))),
      commenter: Rc::new(MonCallbak::new(move || {
        link3.send_message(Msg::CommentCode)
      })),
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    if first_render && RUN_CODE_AT_START {
      ctx.link().send_message(Msg::RunCode);
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::EditorCreated(editor_link) => {
        self.editor = Some(editor_link.clone());
        let code_runner = self.code_runner.clone();
        let code_saver = self.code_saver.clone();
        let commenter = self.commenter.clone();

        editor_link.with_editor(|editor| {
          let run_code = KeyCode::Enter.to_value() | (KeyMod::ctrl_cmd() as u32);
          let save_code = KeyCode::KeyS.to_value() | (KeyMod::ctrl_cmd() as u32);
          let comment_code = KeyCode::UsSlash.to_value() | (KeyMod::ctrl_cmd() as u32);
          let code_runner = (*code_runner).as_ref().unchecked_ref();
          let code_saver = (*code_saver).as_ref().unchecked_ref();
          let commenter = (*commenter).as_ref().unchecked_ref();

          let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
          raw_editor.add_command(run_code.into(), code_runner, None);
          raw_editor.add_command(save_code.into(), code_saver, None);
          raw_editor.add_command(comment_code.into(), commenter, None);
        });

        false
      }
      Msg::RunCode => {
        self.stdout.clear();
        self.code = self.text_model.get_value();
        match Program::from_source(&self.code) {
          Ok(program) => {
            let mut ram = Ram::new(
              program,
              Box::new(self.reader.clone()),
              Box::new(self.writer.clone()),
            );
            self.error = ram.run().err().map(OutputComponentErrors::InterpretError);
            let state: RamState = ram.into();
            self.memory = state.registers;
          }
          Err(e) => self.error = Some(OutputComponentErrors::ParseError(e)),
        };
        true
      }
      Msg::SaveCode => {
        let _ = download_code(&self.text_model.get_value());
        false
      }
      Msg::CommentCode => {
        if let Some(editor) = &self.editor {
          editor.with_editor(|editor| comment_selected_code(editor, &self.text_model));
        }
        true
      }
      Msg::WriterWrote(data) => {
        self.stdout.push_str(&data);
        self.stdout.push('\n');
        true
      }
      Msg::InputChanged(data) => {
        // self.stdin = data;
        // self.stdin.push('\n');
        // self.reader.set_input(self.stdin.clone());
        self.reader.set_input(data);
        true
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let on_editor_created = self.link.callback(Msg::EditorCreated);
    let on_input_changed = self.link.callback(Msg::InputChanged);
    let on_start = self.link.callback(|_| Msg::RunCode);
    let on_pause = self.link.batch_callback(|_| None); // TODO:
    let on_stop = self.link.batch_callback(|_| None); // TODO:
    let on_debug = self.link.batch_callback(|_| None); // TODO:

    html! {
      <main id="ram-web">
        <Header {on_start} {on_stop} {on_pause} {on_debug} />

        <div class="interface">
          <div class="editor-registers">
              <div id="container" class="editor-container">
                <CustomEditor
                  value={INITIAL_CODE}
                  {on_editor_created}
                  text_model={self.text_model.clone()}
                />
              </div>
              <Memory entries={self.memory.clone()} />
          </div>
        </div>

        <div class="console-container">
          <OutputComponent
            error={self.error.clone()}
            output={AttrValue::from(self.stdout.clone())}
          />
          <InputComponent on_change={on_input_changed} default_value={INITIAL_STDIN} />
        </div>

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
