use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    code_editor::CustomEditor,
    code_runner::{CodeRunner, DebugAction},
    header::Header,
    memory::Memory,
    store::{dispatch, Store},
    utils::HydrationGate,
};
#[cfg(not(feature = "ssr"))]
use crate::{
    monaco_tweaks::EditorStoreListener,
    utils::{copy_to_clipboard, get_author},
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
    CopyToClipboard,
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
        let on_copy = ctx.link().callback(|()| Msg::CopyToClipboard);
        let store = &self.store;

        let editor_placeholder = html! {<div id="container" class="editor-container placeholder"/>};
        let line = store.current_debug_line;
        let read_only = store.read_only;

        let main = html! {
            <main id="ram-web">
                <Header {on_run} {on_step} {on_stop} {on_copy} />

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
        };

        #[cfg(not(feature = "ssr"))]
        {
            main
        }
        #[cfg(feature = "ssr")]
        html! {
            <YewduxRoot>{main}</YewduxRoot>
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
            Msg::CopyToClipboard | Msg::DebugStep | Msg::DebugStart => unreachable!(),

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
            #[cfg(not(feature = "ssr"))]
            Msg::CopyToClipboard => {
                let text_model = &self.store.get_model();
                let value = text_model.get_value();
                let encoded_value = urlencoding::encode(&value);
                let author = get_author().map(|it| urlencoding::encode(&it).into_owned());
                let current_page = gloo::utils::window().location().origin();
                let current_page = current_page.expect("no origin");
                let mut url = format!("{}?code={}", current_page, encoded_value);

                if let Some(author) = author {
                    url += "&author=";
                    url += &author;
                }

                log::debug!("Copying to clipboard: {}", url);
                copy_to_clipboard(&url);
            }
        }

        false
    }
}
