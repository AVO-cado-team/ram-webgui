use std::collections::HashSet;

use monaco::api::TextModel;
use ramemu::registers::Registers;

use yew::html::Scope;
use yew::prelude::*;

use crate::code_editor::CustomEditor;
use crate::code_runner::CodeRunner;
use crate::code_runner::DebugAction;
use crate::code_runner::Msg as CodeRunnerMsg;
use crate::header::Header;
use crate::memory::Memory;
use crate::utils::after_hydration::HydrationGate;

pub struct App {
    memory: Registers<i64>,
    text_model: Option<TextModel>,
    code_runner_scope: Option<Scope<CodeRunner>>,
    breakpoints: HashSet<usize>,
    read_only: bool,
}

pub enum Msg {
    SetRunnerScope(Scope<CodeRunner>),
    SetMemory(Registers<i64>),
    DebugStop,
    DebugStep,
    DebugStart,
    SetReadOnly(bool),
    SetTextModel(TextModel),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let run_code = ctx.link().callback(|_: ()| Msg::DebugStart);
        let set_read_only = ctx.link().callback(Msg::SetReadOnly);

        let on_run = ctx.link().callback(|_| Msg::DebugStart);
        let on_stop = ctx.link().callback(|_| Msg::DebugStop);
        let on_step = ctx.link().callback(|_| Msg::DebugStep);
        let on_debug = ctx.link().callback(|_| Msg::DebugStart);

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
                          />
                      </HydrationGate>
                      <Memory entries={self.memory.clone()} />
                  </div>
              </div>

              <CodeRunner
                set_memory={ctx.link().callback(Msg::SetMemory)}
                set_scope={ctx.link().callback(Msg::SetRunnerScope)}
                breakpoints={self.breakpoints.clone()}
                set_read_only={set_read_only}
              />

          </main>
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("App Created");
        Self {
            memory: Default::default(),
            code_runner_scope: None,
            text_model: None,
            breakpoints: Default::default(),
            read_only: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetRunnerScope(scope) => self.code_runner_scope = Some(scope),
            Msg::SetMemory(memory) => {
                self.memory = memory;
                return true;
            }
            Msg::SetTextModel(text_model) => {
                self.text_model = Some(text_model);
                return true;
            }
            Msg::SetReadOnly(read_only) => {
                self.read_only = read_only;
            }
            Msg::DebugStart => {
                if let Some((runner, text_model)) = self.zip_code_runner_and_text_model() {
                    runner.send_message(CodeRunnerMsg::DebugAction(DebugAction::Start(
                        text_model.get_value(),
                    )));
                }
            }
            Msg::DebugStep => {
                if let Some((runner, text_model)) = self.zip_code_runner_and_text_model() {
                    runner.send_message(CodeRunnerMsg::DebugAction(DebugAction::Step(
                        text_model.get_value(),
                    )));
                }
            }
            Msg::DebugStop => {
                if let Some(s) = &self.code_runner_scope {
                    s.send_message(CodeRunnerMsg::DebugAction(DebugAction::Stop));
                }
            }
        }
        false
    }
}

impl App {
    fn zip_code_runner_and_text_model(&self) -> Option<(&Scope<CodeRunner>, &TextModel)> {
        self.code_runner_scope
            .as_ref()
            .zip(self.text_model.as_ref())
    }
}
