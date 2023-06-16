use std::collections::HashSet;

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

use yew::{html::Scope, prelude::*};

const DEFAULT_STDIN: &str = r#" 3 4 "#;

pub enum Msg {
    RunCode(String),
    DebugStart(String),
    DebugStep,
    DebugStop,
    DebugContinue,
    WriterWrote(String),
    InputChanged(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub set_memory: Callback<Registers<i64>>,
    pub set_scope: Callback<Scope<CodeRunner>>,
    pub breakpoints: HashSet<usize>,
    pub set_read_only: Callback<bool>,
}

pub struct CodeRunner {
    error: Option<OutputComponentErrors>,
    stdout: String,
    reader: CustomReader,
    writer: CustomWriter,
    debug: Option<DebugState>,
}

struct DebugState {
    ram: Ram,
}

impl Component for CodeRunner {
    type Message = Msg;
    type Properties = Props;

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_input_changed = ctx.link().callback(Msg::InputChanged);

        let default_stdin =
            get_from_local_storage("stdin").unwrap_or_else(|| DEFAULT_STDIN.to_string());

        html! {
            <div class="console-container">
              <OutputComponent
                error={self.error.clone()}
                output={self.stdout.clone()}
              />
              <InputComponent
                on_change={on_input_changed}
                default_value={default_stdin}
              />
          </div>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        monaco::workers::ensure_environment_set();

        ctx.props().set_scope.emit(ctx.link().clone());
        let stdin = get_from_local_storage("stdin").unwrap_or_else(|| DEFAULT_STDIN.to_string());

        let reader = CustomReader::new(stdin);

        CodeRunner {
            error: None,
            debug: None,
            stdout: Default::default(),
            reader,
            writer: CustomWriter::new(ctx.link().callback(Msg::WriterWrote)),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().set_scope != old_props.set_scope {
            ctx.props().set_scope.emit(ctx.link().clone());
        }

        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RunCode(code) => {
                log::info!("Run Code");

                if self.debug.is_some() {
                    ctx.link().send_message(Msg::DebugContinue);
                    return false;
                }

                self.stdout.clear();
                match Program::from_source(&code) {
                    Ok(program) => {
                        let mut ram = Ram::new(
                            program,
                            Box::new(self.reader.clone()),
                            Box::new(self.writer.clone()),
                        );
                        self.error = ram.run().err().map(OutputComponentErrors::InterpretError);
                        let state: RamState = ram.into();

                        ctx.props().set_memory.emit(state.registers);
                    }
                    Err(e) => self.error = Some(OutputComponentErrors::ParseError(e)),
                };
            }
            Msg::DebugStart(code) => {
                log::info!("Debug Start");

                self.stdout.clear();
                match Program::from_source(&code) {
                    Ok(program) => {
                        let ram = Ram::new(
                            program,
                            Box::new(self.reader.clone()),
                            Box::new(self.writer.clone()),
                        );

                        self.debug = Some(DebugState { ram });

                        ctx.props().set_read_only.emit(true);

                        ctx.link().send_message(Msg::DebugStep);
                    }
                    Err(e) => self.error = Some(OutputComponentErrors::ParseError(e)),
                };
            }
            Msg::DebugStep => {
                log::info!("Debug Step");

                if let Some(debug) = self.debug.as_mut() {
                    debug.ram.next();

                    let state: RamState = (&debug.ram).into();

                    ctx.props().set_memory.emit(state.registers);
                    // ctx.props().set_line.emit(state.line);

                    if state.halt {
                        ctx.link().send_message(Msg::DebugStop);
                    }
                }
            }
            Msg::DebugContinue => {
                log::info!("Debug Continue");

                if let Some(debug) = self.debug.as_mut() {
                    for result in debug.ram.by_ref() {
                        let breakpoints = &ctx.props().breakpoints;
                        match result {
                            Ok(state) if breakpoints.contains(&state.line) => break,
                            Err(_) => break,
                            _ => {}
                        }
                    }

                    let state: RamState = (&debug.ram).into();

                    ctx.props().set_memory.emit(state.registers);
                    // ctx.props().set_line.emit(state.line);

                    if state.halt {
                        ctx.link().send_message(Msg::DebugStop);
                    }
                }
            }
            Msg::DebugStop => {
                log::info!("Debug Stop");

                ctx.props().set_read_only.emit(false);

                if let Some(debug) = self.debug.take() {
                    let state: RamState = debug.ram.into();
                    self.error = state.error.map(OutputComponentErrors::InterpretError);

                    ctx.props().set_memory.emit(state.registers);
                    // ctx.props().set_line.emit(state.line);
                }
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
}
