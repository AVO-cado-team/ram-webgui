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

mod private {
    #[derive(Clone, Copy, Debug)]
    pub struct PrivateZst;
}

#[cfg(debug_assertions)]
const DELAY_BETWEEN_STEPS: Duration = Duration::from_millis(10);
#[cfg(not(debug_assertions))]
const DELAY_BETWEEN_STEPS: Duration = Duration::from_millis(10);

pub enum Msg {
    WriterWrote(String),
    InputChanged(String),
    DebugAction(DebugAction),
}

#[derive(Debug)]
pub enum DebugAction {
    Start(String),
    Step(String),
    Stop,

    StepInner(private::PrivateZst),
    ContinueChain(private::PrivateZst),
}

type State = Option<(StateKind, Ram)>;

#[derive(Copy, Clone, PartialEq)]
enum StateKind {
    WaitOnContinue,
    Pause,
}

use StateKind::{Pause, WaitOnContinue};

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
    debug: State,
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

        let reader = CustomReader::new("");

        Self {
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
        let action = match msg {
            Msg::DebugAction(action) => action,
            Msg::WriterWrote(data) => {
                self.stdout.push_str(&data);
                self.stdout.push('\n');
                return true;
            }
            Msg::InputChanged(data) => {
                log::info!("Input changed, {}", &data);
                save_to_local_storage("stdin", &data);
                self.reader.set_input(&data);
                return true;
            }
        };

        self.debug = match (self.debug.take(), &action) {
            (debug, DebugAction::Stop) => self.debug_stop(ctx, debug),
            (Some((Pause, ram)), DebugAction::Start(_))
            | (Some((WaitOnContinue, ram)), DebugAction::ContinueChain(_)) => {
                self.debug_continue(ctx, ram)
            }
            (None, DebugAction::Start(_) | DebugAction::Step(_)) => self.debug_start(ctx, action),
            (Some((_, ram)), DebugAction::Step(_) | DebugAction::StepInner(_)) => self.debug_step(ctx, ram),
            // DebugContinue is ignored if not in WaitOnContinue
            // In Pause or out of debug mode, it could arrive as a result of async
            (debug, DebugAction::ContinueChain(_)) | // already waiting for new step
            (debug @ Some((WaitOnContinue, _)), _) => debug,
            (_, DebugAction::StepInner(_)) => panic!("Dispatched `DebugStepInner` without `debug_start`"),
        };
        true
    }
}

impl CodeRunner {
    fn debug_start(&mut self, ctx: &Context<Self>, action: DebugAction) -> State {
        log::info!("Debug Start");

        self.stdout.clear();

        let (code, message, state) = match action {
            DebugAction::Start(code) => (
                code,
                DebugAction::ContinueChain(private::PrivateZst),
                WaitOnContinue,
            ),
            DebugAction::Step(code) => (code, DebugAction::StepInner(private::PrivateZst), Pause),
            state => panic!("Called `debug_start` in {state:?}"),
        };

        match Program::from_source(&code) {
            Ok(program) => {
                let ram = Ram::new(
                    program,
                    Box::new(self.reader.clone()),
                    Box::new(self.writer.clone()),
                );

                ctx.props().set_read_only.emit(true);

                ctx.link().send_message(Msg::DebugAction(message));
                Some((state, ram))
            }
            Err(e) => {
                self.error = Some(OutputComponentErrors::ParseError(e));
                None
            }
        }
    }

    #[allow(clippy::unnecessary_wraps, clippy::unused_self)]
    fn debug_step(&self, ctx: &Context<Self>, mut ram: Ram) -> State {
        log::info!("Debug Step");

        if ram.next().is_some() {
            let state: RamState = ram.as_ref().into();

            ctx.props().set_memory.emit(state.registers);
            // ctx.props().set_line.emit(state.line);
        } else {
            ctx.link().send_message(Msg::DebugAction(DebugAction::Stop));
        }

        Some((Pause, ram))
    }

    #[allow(clippy::unnecessary_wraps, clippy::unused_self)]
    fn debug_continue(&self, ctx: &Context<Self>, mut ram: Ram) -> State {
        log::info!("Debug Continue");

        let breakpoints = &ctx.props().breakpoints;
        let kind;

        match ram.next() {
            Some(state) if breakpoints.contains(&state.line) => kind = Pause,
            None => {
                ctx.link().send_message(Msg::DebugAction(DebugAction::Stop));
                return Some((Pause, ram)); // Not `None` to give `debug_stop` a state to work with
            }
            Some(_) => wasm_bindgen_futures::spawn_local({
                let scope = ctx.link().clone();
                kind = WaitOnContinue;
                async move {
                    sleep(DELAY_BETWEEN_STEPS).await;
                    scope.send_message(Msg::DebugAction(DebugAction::ContinueChain(private::PrivateZst)));
                }
            }),
        };
        let state: RamState = ram.as_ref().into();
        ctx.props().set_memory.emit(state.registers);
        // ctx.props().set_line.emit(state.line);

        Some((kind, ram))
    }

    fn debug_stop(&mut self, ctx: &Context<Self>, debug: State) -> State {
        log::info!("Debug Stop");

        let (_, ram) = debug?;

        let state: RamState = ram.into();
        self.error = state.error.map(OutputComponentErrors::InterpretError);

        ctx.props().set_read_only.emit(false);
        ctx.props().set_memory.emit(state.registers);
        // ctx.props().set_line.emit(state.line);
        None
    }
}
