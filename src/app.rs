use std::collections::HashSet;

use monaco::api::TextModel;
use ramemu::registers::Registers;

use yew::prelude::*;

use crate::code_editor::CustomEditor;
use crate::code_runner::CodeRunner;
use crate::code_runner::DebugAction;
use crate::header::Header;
use crate::memory::Memory;
use crate::utils::after_hydration::HydrationGate;

#[derive(Default)]
pub struct App {
    memory: Registers<i64>,
    text_model: Option<TextModel>,
    code_runner_dispatch: Option<Callback<DebugAction>>,
    breakpoints: HashSet<usize>,
    read_only: bool,
    line: usize
}

pub enum Msg {
    SetRunnerDispatch(Callback<DebugAction>),
    SetMemory(Registers<i64>),
    DebugStop,
    DebugStep,
    DebugStart,
    SetReadOnly(bool),
    SetTextModel(TextModel),
    SetLine(usize)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let run_code = ctx.link().callback(|()| Msg::DebugStart);
        let set_read_only = ctx.link().callback(Msg::SetReadOnly);

        let on_run = ctx.link().callback(|()| Msg::DebugStart);
        let on_stop = ctx.link().callback(|()| Msg::DebugStop);
        let on_step = ctx.link().callback(|()| Msg::DebugStep);
        let on_debug = ctx.link().callback(|()| Msg::DebugStart);

        let set_text_model = ctx.link().callback(Msg::SetTextModel);
        let editor_placeholder = html! {<div id="container" class="editor-container placeholder"/>};

        html! {
          <main id="ram-web">
              <Header {on_run} {on_step} {on_stop} {on_debug} />

              <div class="interface">
                  <div class="editor-registers">
                      <HydrationGate placeholder={editor_placeholder}>
                          <CustomEditor
                              read_only={self.read_only}
                              set_text_model={set_text_model}
                              run_code={run_code}
                              line={self.line}
                          />
                      </HydrationGate>
                      <Memory entries={self.memory.clone()} />
                  </div>
              </div>

              <CodeRunner
                  memory_setter={ctx.link().callback(Msg::SetMemory)}
                  dispatch_setter={ctx.link().callback(Msg::SetRunnerDispatch)}
                  breakpoints={self.breakpoints.clone()}
                  read_only_setter={set_read_only}
                  line_setter={ctx.link().callback(Msg::SetLine)}
              />

          </main>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("App Created");
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let zip_dispatch_and_text_model = || {
            self.code_runner_dispatch
                .as_ref()
                .zip(self.text_model.as_ref())
        };

        match msg {
            Msg::SetRunnerDispatch(dispatch) => self.code_runner_dispatch = Some(dispatch),
            Msg::SetReadOnly(read_only) => self.read_only = read_only,
            Msg::SetLine(line) => {
                self.line = line;
                return true;
            }
            Msg::SetMemory(memory) => {
                self.memory = memory;
                return true;
            }
            Msg::SetTextModel(text_model) => {
                self.text_model = Some(text_model);
                return true;
            }

            Msg::DebugStart => {
                if let Some((dispatch, text_model)) = zip_dispatch_and_text_model() {
                    dispatch.emit(DebugAction::Start(text_model.get_value()));
                }
            }
            Msg::DebugStep => {
                if let Some((dispatch, text_model)) = zip_dispatch_and_text_model() {
                    dispatch.emit(DebugAction::Step(text_model.get_value()));
                }
            }
            Msg::DebugStop => {
                if let Some(dispatch) = &self.code_runner_dispatch {
                    dispatch.emit(DebugAction::Stop);
                }
            }
        }

        false
    }
}
