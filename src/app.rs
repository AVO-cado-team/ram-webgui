use crate::code_editor::CustomEditor;
use crate::header::Header;
use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use crate::io::input::InputComponent;
use crate::io::output::OutputComponent;
use crate::io::output::OutputComponentErrors;
use crate::memory::Memory;
use crate::show_content::show_content;

use std::rc::Rc;

use ramemu::registers::Registers;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::html::Scope;
use yew::prelude::*;

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
  code_runner: Option<Rc<Closure<dyn Fn()>>>,
  memory: Registers<i64>,
  error: Option<OutputComponentErrors>,
  reader: CustomReader,
  writer: CustomWriter,
}

pub enum Msg {
  EditorCreated(CodeEditorLink),
  RunCode,
  WriterWrote(String),
  InputChanged(String),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      link: ctx.link().clone(),
      text_model: TextModel::create(INITIAL_CODE, Some("ram"), None).unwrap(),
      code: String::from(INITIAL_CODE),
      // stdin: Default::default(),
      stdout: Default::default(),
      memory: Default::default(),
      error: None,
      reader: CustomReader::new(INITIAL_STDIN.to_string()),
      writer: CustomWriter::new(ctx.link().callback(Msg::WriterWrote)),
      code_runner: None,
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    // if first_render {
    //   show_content();
    // }
    if first_render && RUN_CODE_AT_START {
      ctx.link().send_message(Msg::RunCode);
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::EditorCreated(editor_link) => {
        log::info!("Editor created");
        let link = self.link.clone();

        let code_runner = self
          .code_runner
          .get_or_insert_with(|| {
            Rc::new(Closure::<dyn Fn()>::new(move || {
              link.send_message(Msg::RunCode);
            }))
          })
          .clone();

        editor_link.with_editor(|editor| {
          let keycode = KeyCode::Enter.to_value() | (KeyMod::ctrl_cmd() as u32);
          let raw_editor: &IStandaloneCodeEditor = editor.as_ref();

          raw_editor.add_command(
            keycode.into(),
            (*code_runner).as_ref().unchecked_ref(),
            None,
          );
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
            self.error = ram
              .run()
              .err()
              .map(OutputComponentErrors::InterpretError);
          }
          Err(e) => self.error = Some(OutputComponentErrors::ParseError(e)),
        };
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
          <InputComponent on_change={on_input_changed} default_value={INITIAL_STDIN} />
          <OutputComponent
            error={self.error.clone()}
            output={AttrValue::from(self.stdout.clone())}
          />
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
