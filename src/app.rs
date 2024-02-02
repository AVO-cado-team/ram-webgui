use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

#[cfg(not(feature = "ssr"))]
use crate::monaco_tweaks::EditorStoreListener;
use crate::{
    code_editor::CustomEditor,
    code_runner::{CodeRunner, DebugAction},
    header::Header,
    memory::Memory,
    store::{dispatch, Store},
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

        html! {
        <YewduxRoot>
            <main id="ram-web">
                <Header {on_run} {on_step} {on_stop} />

                <div class="interface">
                    <div class="editor-registers">
                        <HydrationGate placeholder={editor_placeholder}>
                            <CustomEditor {read_only} {run_code} {line} />
                        </HydrationGate>
                        <Memory />
                    </div>
                </div>

                <CodeRunner dispatch_setter={ctx.link().callback(Msg::SetRunnerDispatch)} />

            </main>
        </YewduxRoot>
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("App Created");

        // listner_init

        let dispatch = dispatch();
        #[cfg(not(feature = "ssr"))]
        init_listener(EditorStoreListener::default(), dispatch.context());

        let on_change = ctx.link().callback(Msg::SetStore);
        let dispatch = dispatch.subscribe(on_change);

        Self {
            code_runner_dispatch: Default::default(),
            store: dispatch.get(),
            _dispatch: dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetRunnerDispatch(dispatch) => self.code_runner_dispatch = dispatch,
            Msg::SetStore(store) => {
                self.store = store;
                return true;
            }

            #[cfg(feature = "ssr")]
            Msg::DebugStep | Msg::DebugStart => {
                panic!("Debugging is not supported in server side rendering")
            }
            #[cfg(not(feature = "ssr"))]
            Msg::DebugStart => {
                let text_model = &self.store.get_model();
                self.code_runner_dispatch
                    .emit(DebugAction::Start(text_model.get_value()));
            }
            #[cfg(not(feature = "ssr"))]
            Msg::DebugStep => {
                let text_model = &self.store.get_model();
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
