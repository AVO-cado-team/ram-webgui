use std::collections::HashSet;

use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use crate::io::input::InputComponent;
use crate::io::output::OutputComponent;
use crate::io::output::OutputComponentErrors;
use crate::utils::save_to_local_storage;
use crate::utils::sleep;

use ramemu::program::Program;
use ramemu::ram::Ram;
use ramemu::ram::RamState;
use ramemu::registers::Registers;

use yew::{html::Scope, prelude::*};

use std::time::Duration;

#[cfg(debug_assertions)]
const DELAY_BETWEEN_STEPS: Duration = Duration::from_millis(10);
#[cfg(not(debug_assertions))]
const DELAY_BETWEEN_STEPS: Duration = Duration::from_millis(10);

pub enum Msg {
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
    debug: Option<State>,
}

#[derive(Clone, PartialEq)]
enum StateKind {
    DebugContinue,
    DebugPause,
}

use StateKind::*;

struct State {
    kind: StateKind,
    ram: Ram,
}

impl State {
    fn new(kind: StateKind, ram: Ram) -> State {
        State { kind, ram }
    }
}

impl Component for CodeRunner {
    type Message = Msg;
    type Properties = Props;

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
              />
          </div>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().set_scope.emit(ctx.link().clone());

        let reader = CustomReader::new(String::new());

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

                        ctx.props().set_read_only.emit(true);

                        if cfg!(debug_assertions) {
                            self.debug = Some(State::new(DebugPause, ram));
                            ctx.link().send_message(Msg::DebugStep);
                        } else {
                            self.debug = Some(State::new(DebugContinue, ram));
                            ctx.link().send_message(Msg::DebugContinue);
                        }
                    }
                    Err(e) => self.error = Some(OutputComponentErrors::ParseError(e)),
                };
            }
            Msg::DebugStep => 'block: {
                log::info!("Debug Step");

                //  NOTE: by the ent of step kind would not change.
                let Some(State { kind: DebugPause, ram }) = self.debug.as_mut() else {
                    break 'block
                };

                if ram.next().is_some() {
                    let state: RamState = ram.into();

                    ctx.props().set_memory.emit(state.registers);
                    // ctx.props().set_line.emit(state.line);
                } else {
                    ctx.link().send_message(Msg::DebugStop);
                }
            }
            Msg::DebugContinue => 'block: {
                log::info!("Debug Continue");
                let Some(State {
                    kind: kind @ DebugContinue,
                    ram,
                }) = self.debug.as_mut() else { break 'block };


                let breakpoints = &ctx.props().breakpoints;

                match ram.next() {
                    Some(state) if breakpoints.contains(&state.line) => *kind = DebugPause,
                    None => ctx.link().send_message(Msg::DebugStop),
                    Some(_) => {
                        let scope = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            sleep(DELAY_BETWEEN_STEPS).await;
                            scope.send_message(Msg::DebugContinue);
                        });
                    }
                };

                let state: RamState = ram.into();
                ctx.props().set_memory.emit(state.registers);
                // ctx.props().set_line.emit(state.line);
            }
            Msg::DebugStop => 'block: {
                log::info!("Debug Stop");

                let Some(State { kind: _, ram }) = self.debug.take() else {
                    break 'block
                };

                let state: RamState = ram.into();
                self.error = state.error.map(OutputComponentErrors::InterpretError);

                ctx.props().set_read_only.emit(false);
                ctx.props().set_memory.emit(state.registers);
                // ctx.props().set_line.emit(state.line);
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
