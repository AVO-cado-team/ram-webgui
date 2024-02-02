use std::rc::Rc;

use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use crate::io::input::InputComponent;
use crate::io::output::OutputComponent;
use crate::io::output::OutputComponentErrors;
use crate::store::dispatch;
use crate::store::Store;

use ramemu::program::Program;
use ramemu::ram::Ram;
use ramemu::ram::RamState;

use yew::prelude::*;
use yewdux::Dispatch;

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
    DebugAction(DebugAction),
    SetStore(Rc<Store>),
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
    pub dispatch_setter: Callback<Callback<DebugAction>>,
}

pub struct CodeRunner {
    stdout: String,
    writer: CustomWriter,
    debug: State,
    store: Rc<Store>,
    _dispatch: Dispatch<Store>,
}

impl Component for CodeRunner {
    type Message = Msg;
    type Properties = Props;

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let error = self.store.error.clone();
        html! {
            <div class="console-container">
              <OutputComponent
                {error}
                output={self.stdout.clone()}
              />
              <InputComponent />
          </div>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .dispatch_setter
            .emit(ctx.link().callback(Msg::DebugAction));

        let on_change = ctx.link().callback(Msg::SetStore);
        let dispatch = dispatch().subscribe(on_change);

        Self {
            debug: None,
            stdout: Default::default(),
            writer: CustomWriter::new(ctx.link().callback(Msg::WriterWrote)),
            store: dispatch.get(),
            _dispatch: dispatch,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().dispatch_setter != old_props.dispatch_setter {
            ctx.props()
                .dispatch_setter
                .emit(ctx.link().callback(Msg::DebugAction));
        }

        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let action = match msg {
            Msg::SetStore(store) => {
                self.store = store;
                return true;
            }
            Msg::DebugAction(action) => action,
            Msg::WriterWrote(data) => {
                self.stdout.push_str(&data);
                self.stdout.push('\n');
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
                    Box::new(CustomReader::new(&self.store.stdin)),
                    Box::new(self.writer.clone()),
                );

                dispatch().reduce_mut(|s: &mut Store| s.read_only = true);

                ctx.link().send_message(Msg::DebugAction(message));
                Some((state, ram))
            }
            Err(e) => {
                dispatch().reduce_mut(move |store: &mut Store| {
                    store.error = Some(OutputComponentErrors::ParseError(e));
                });
                None
            }
        }
    }

    #[allow(clippy::unnecessary_wraps, clippy::unused_self)]
    fn debug_step(&self, ctx: &Context<Self>, mut ram: Ram) -> State {
        log::debug!("Debug Step");

        match ram.next() {
            Some(state) => {
                let registers = state.registers;
                let line = state.line;
                dispatch().reduce_mut(|s: &mut Store| {
                    s.set_registers(registers);
                    s.current_debug_line = line;
                });
            }
            None => ctx.link().send_message(Msg::DebugAction(DebugAction::Stop)),
        }

        Some((Pause, ram))
    }

    #[allow(clippy::unnecessary_wraps, clippy::unused_self)]
    fn debug_continue(&self, ctx: &Context<Self>, mut ram: Ram) -> State {
        log::debug!("Debug Continue");

        let breakpoints = &self.store.breakpoints;
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
                    yew::platform::time::sleep(DELAY_BETWEEN_STEPS).await;
                    scope.send_message(Msg::DebugAction(DebugAction::ContinueChain(
                        private::PrivateZst,
                    )));
                }
            }),
        };
        let state: RamState = ram.as_ref().into();
        let registers = state.registers;
        let line = state.line;

        dispatch().reduce_mut(|s: &mut Store| {
            s.set_registers(registers);
            s.current_debug_line = line;
        });

        Some((kind, ram))
    }

    fn debug_stop(&mut self, _ctx: &Context<Self>, debug: State) -> State {
        log::info!("Debug Stop");

        let (_, ram) = debug?;

        let state: RamState = ram.into();

        let registers = state.registers;
        let error = state.error;

        dispatch().reduce_mut(|s: &mut Store| {
            s.set_registers(registers);
            s.read_only = false;
            s.current_debug_line = 0;
            s.error = error.map(OutputComponentErrors::InterpretError);
        });

        None
    }
}
