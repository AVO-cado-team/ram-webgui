use crate::code_editor::CustomEditor;
use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use crate::io::input::InputComponent;
use crate::io::output::OutputComponent;

use std::rc::Rc;

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
halt
"#;

const INITIAL_STDIN: &str = r#"
3
"#;

const RUN_CODE_AT_START: bool = false;

pub struct App {
  link: Scope<Self>,
  text_model: TextModel,
  code: String,
  interpretator_output: String,
  // stdin: String,
  stdout: String,
  code_runner: Option<Rc<Closure<dyn Fn()>>>,
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
      interpretator_output: Default::default(),
      // stdin: Default::default(),
      stdout: Default::default(),
      reader: CustomReader::new(INITIAL_STDIN.to_string()),
      writer: CustomWriter::new(ctx.link().callback(Msg::WriterWrote)),
      code_runner: None,
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
        self.interpretator_output = match Program::from_source(&self.code) {
          Ok(program) => {
            let mut ram = Ram::new(
              program,
              Box::new(self.reader.clone()),
              Box::new(self.writer.clone()),
            );
            format!("{:?}", ram.run())
          }
          Err(e) => format!("{:?}", e),
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

    html! {
      <div id="code-wrapper">
        <div id="code-editor">
          <CustomEditor
            value={INITIAL_CODE}
            {on_editor_created}
            text_model={self.text_model.clone()}
          />
        </div>
        <InputComponent on_change={on_input_changed} default_value={INITIAL_STDIN} />
        <OutputComponent output={AttrValue::from(self.stdout.clone())} />
        <div id="event-log-wrapper">
          <div id="event-log">
            <h2>{"Code (press CTRL+Enter / Command+Enter to view)"}</h2>
            <pre> {self.interpretator_output.clone()} </pre>
          </div>
        </div>
      </div>
    }
  }
}
