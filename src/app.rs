use crate::code_editor::CustomEditor;
use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use std::io::BufReader;
use std::rc::Rc;

use monaco::sys::KeyCode;
use monaco::sys::KeyMod;
use monaco::{api::TextModel, sys::editor::IStandaloneCodeEditor, yew::CodeEditorLink};

use wasm_bindgen::closure::Closure;
use yew::html::Scope;
use yew::prelude::*;

use crate::io::output::OutputComponent;
use ramemu::program::Program;
use ramemu::ram::Ram;

use wasm_bindgen::JsCast;

const INITIAL_CODE: &str = r#"
write =3
halt

"#;

pub struct App {
  link: Scope<Self>,
  text_model: TextModel,
  code: String,
  interpretator_output: String,
  stdout: String,
  js_closure: Option<Rc<Closure<dyn Fn()>>>,
  reader: CustomReader,
  writer: CustomWriter,
}

pub enum Msg {
  EditorCreated(CodeEditorLink),
  CodeChanged,
  WriterWrote(String),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let text_model = TextModel::create(INITIAL_CODE, Some("ram"), None).unwrap();
    let code = String::from(INITIAL_CODE);
    let interpretator_output = String::from("");
    let stdout = String::from("");
    let reader = CustomReader::new();
    let writer = CustomWriter::new(ctx.link().callback(Msg::WriterWrote));

    Self {
      link: ctx.link().clone(),
      text_model,
      code,
      interpretator_output,
      stdout,
      reader,
      writer,
      js_closure: None,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::EditorCreated(editor_link) => {
        let link = self.link.clone();
        let js_closure = Rc::new(Closure::<dyn Fn()>::new(move || {
          link.send_message(Msg::CodeChanged);
        }));

        self.js_closure = Some(js_closure.clone());

        editor_link.with_editor(|editor| {
          let keycode = KeyCode::Enter.to_value() | (KeyMod::ctrl_cmd() as u32);
          let raw_editor: &IStandaloneCodeEditor = editor.as_ref();

          raw_editor.add_command(keycode.into(), (*js_closure).as_ref().unchecked_ref(), None);
        });

        false
      }
      Msg::CodeChanged => {
        self.stdout.clear();
        self.code = self.text_model.get_value();
        self.interpretator_output = match Program::from_source(&self.code) {
          Ok(program) => {
            let reader = BufReader::new(std::io::empty());
            let mut ram = Ram::new(program, Box::new(reader), Box::new(self.writer.clone()));
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
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let on_editor_created = self.link.callback(Msg::EditorCreated);

    html! {
      <div id="code-wrapper">
        <div id="code-editor">
          <CustomEditor {on_editor_created} text_model={self.text_model.clone()} />
        </div>
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
