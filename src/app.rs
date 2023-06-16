use std::collections::HashSet;

use monaco::api::TextModel;
use ramemu::registers::Registers;

use yew::html::Scope;
use yew::prelude::*;

use crate::code_editor::CustomEditor;
use crate::code_runner::CodeRunner;
use crate::code_runner::Msg as CodeRunnerMsg;
use crate::header::Header;
use crate::memory::Memory;
use crate::utils::get_from_local_storage;

pub struct App {
    memory: Registers<i64>,
    text_model: TextModel,
    default_code: String,
    code_runner_scope: Option<Scope<CodeRunner>>,
    breakpoints: HashSet<usize>,
    read_only: bool,
}

pub enum Msg {
    SetRunnerScope(Scope<CodeRunner>),
    SetMemory(Registers<i64>),
    RunCode,
    DebugStop,
    DebugStep,
    DebugStart,
    SetReadOnly(bool),
}

const DEFAULT_CODE: &str = r#"
read 1
write 1
read 2
write 2
halt
"#;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let run_code = ctx.link().callback(|_| Msg::RunCode);
        let set_read_only = ctx.link().callback(Msg::SetReadOnly);

        let on_run = ctx.link().callback(|_| Msg::RunCode);
        let on_stop = ctx.link().callback(|_| Msg::DebugStop);
        let on_step = ctx.link().callback(|_| Msg::DebugStep);
        let on_debug = ctx.link().callback(|_| Msg::DebugStart);

        html! {
          <main id="ram-web">
            <Header {on_run} {on_step} {on_stop} {on_debug} />

            <div class="interface">
              <div class="editor-registers">
                  <CustomEditor
                    read_only={self.read_only}
                    value={self.default_code.clone()}
                    text_model={self.text_model.clone()}
                    run_code={run_code}
                  />
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
        let code = get_from_local_storage("code").unwrap_or_else(|| DEFAULT_CODE.to_string());
        let text_model = TextModel::create(code.as_str(), Some("ram"), None)
            .expect("Failed to create text model");

        Self {
            memory: Default::default(),
            code_runner_scope: None,
            text_model,
            default_code: code,
            breakpoints: Default::default(),
            read_only: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetRunnerScope(scope) => self.code_runner_scope = Some(scope),
            Msg::SetMemory(memory) => {
                self.memory = memory;
                return true;
            }
            Msg::SetReadOnly(read_only) => {
                self.read_only = read_only;
            }
            Msg::RunCode => {
                if let Some(s) = &self.code_runner_scope {
                    s.send_message(CodeRunnerMsg::RunCode(self.text_model.get_value()));
                }
            }
            Msg::DebugStart => {
                if let Some(s) = &self.code_runner_scope {
                    s.send_message(CodeRunnerMsg::DebugStart(self.text_model.get_value()));
                }
            }
            Msg::DebugStop => {
                if let Some(s) = &self.code_runner_scope {
                    s.send_message(CodeRunnerMsg::DebugStop);
                }
            }
            Msg::DebugStep => {
                if let Some(s) = &self.code_runner_scope {
                    s.send_message(CodeRunnerMsg::DebugStep);
                }
            }
        }
        false
    }
}
