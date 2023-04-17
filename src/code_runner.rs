use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use crate::io::input::InputComponent;
use crate::io::output::OutputComponent;
use crate::io::output::OutputComponentErrors;
use crate::utils::get_from_local_storage;
use crate::utils::save_to_local_storage;
use ramemu::program::Program;
use ramemu::ram::Ram;
use ramemu::ram::RamState;
use ramemu::registers::Registers;
// use wasm_bindgen::prelude::Closure;
use yew::{html::Scope, prelude::*};

const INITIAL_STDIN: &str = r#" 3 4 "#;

pub enum Msg {
  RunCode,
  WriterWrote(String),
  InputChanged(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub code: AttrValue,
  pub set_memory: Callback<Registers<i64>>,
  pub set_scope: Callback<Scope<CodeRunner>>,
}

pub struct CodeRunner {
  error: Option<OutputComponentErrors>,
  default_stdin: String,
  stdout: String,
  code: String,
  set_memory: Callback<Registers<i64>>,
  reader: CustomReader,
  writer: CustomWriter,
}

impl Component for CodeRunner {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    ctx.props().set_scope.emit(ctx.link().clone());
    let stdin = get_from_local_storage("stdin").unwrap_or_else(|| INITIAL_STDIN.to_string());
    CodeRunner {
      error: None,
      default_stdin: stdin.clone(),
      stdout: Default::default(),
      code: ctx.props().code.to_string(),
      set_memory: ctx.props().set_memory.clone(),
      reader: CustomReader::new(stdin),
      writer: CustomWriter::new(ctx.link().callback(Msg::WriterWrote)),
    }
  }

  fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
    if ctx.props().code != self.code {
      self.code = ctx.props().code.to_string();
    } else if ctx.props().set_memory != self.set_memory {
      self.set_memory = ctx.props().set_memory.clone();
    } else if ctx.props().set_scope != old_props.set_scope {
      ctx.props().set_scope.emit(ctx.link().clone());
    }
    false
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::RunCode => {
        log::info!("Run Code");
        self.stdout.clear();
        match Program::from_source(&self.code) {
          Ok(program) => {
            let mut ram = Ram::new(
              program,
              Box::new(self.reader.clone()),
              Box::new(self.writer.clone()),
            );
            self.error = ram.run().err().map(OutputComponentErrors::InterpretError);
            let state: RamState = ram.into();

            self.set_memory.emit(state.registers);
          }
          Err(e) => self.error = Some(OutputComponentErrors::ParseError(e)),
        };
      }
      Msg::WriterWrote(data) => {
        self.stdout.push_str(&data);
        self.stdout.push('\n');
      }
      Msg::InputChanged(data) => {
        log::info!("Input changed, {}", &data);
        save_to_local_storage("stdin", &data);
        self.reader.set_input(data);
      }
    };
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let on_input_changed = ctx.link().callback(Msg::InputChanged);
    html! {
        <div class="console-container">
          <OutputComponent
            error={self.error.clone()}
            output={self.stdout.clone()}
          />
          <InputComponent
            on_change={on_input_changed}
            default_value={self.default_stdin.clone()}
          />
      </div>
    }
  }
}
