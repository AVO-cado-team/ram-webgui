use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    code_editor::CustomEditor,
    code_runner::{CodeRunner, DebugAction},
    header::Header,
    memory::Memory,
    store::Store,
    utils::HydrationGate,
};

pub struct App {
    code_runner_dispatch: Callback<DebugAction>,
    store: Rc<Store>,
    _dispatch: Dispatch<Store>,
}

pub enum Msg {
    SetRunnerDispatch(Callback<DebugAction>),
    DebugStop,
    DebugStep,
    DebugStart,
    SetStore(Rc<Store>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let run_code = ctx.link().callback(|()| Msg::DebugStart);

        let on_run = ctx.link().callback(|()| Msg::DebugStart);
        let on_stop = ctx.link().callback(|()| Msg::DebugStop);
        let on_step = ctx.link().callback(|()| Msg::DebugStep);
        let store = &self.store;

        let editor_placeholder = html! {<div id="container" class="editor-container placeholder"/>};
        let line = store.current_debug_line;
        let read_only = store.read_only;
        let text_model = store.get_model().clone();

        html! {
          <main id="ram-web">
              <Header {on_run} {on_step} {on_stop} />

              <div class="interface">
                  <div class="editor-registers">
                      <HydrationGate placeholder={editor_placeholder}>
                          <CustomEditor {text_model} {read_only} {run_code} {line} />
                      </HydrationGate>
                      <Memory />
                  </div>
              </div>

              <CodeRunner dispatch_setter={ctx.link().callback(Msg::SetRunnerDispatch)} />

          </main>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("App Created");

        let on_change = ctx.link().callback(Msg::SetStore);
        let dispatch = Dispatch::global().subscribe(on_change);

        Self {
            code_runner_dispatch: Default::default(),
            store: dispatch.get(),
            _dispatch: dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let text_model = &self.store.get_model();
        match msg {
            Msg::SetRunnerDispatch(dispatch) => self.code_runner_dispatch = dispatch,
            Msg::SetStore(store) => {
                self.store = store;
                return true;
            }

            Msg::DebugStart => {
                self.code_runner_dispatch
                    .emit(DebugAction::Start(text_model.get_value()));
            }
            Msg::DebugStep => {
                self.code_runner_dispatch
                    .emit(DebugAction::Step(text_model.get_value()));
            }
            Msg::DebugStop => {
                self.code_runner_dispatch.emit(DebugAction::Stop);
            }
        }

        false
    }
}
