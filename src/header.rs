use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

use crate::about_popup::AboutPopup;

#[derive(Default, Clone, PartialEq, Properties)]
pub struct Props {
    pub on_run: Callback<()>,
    pub on_step: Callback<()>,
    pub on_stop: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let popup_ref = use_node_ref();
    let popup_button_ref = use_node_ref();
    let show_popup = use_state_eq(|| false);
    let event_listener = use_state(|| None);

    let callback = use_callback(
        (
            show_popup.clone(),
            popup_ref.clone(),
            popup_button_ref.clone(),
        ),
        |event: web_sys::MouseEvent, (show_popup, popup_ref, about_us)| {
            let (popup_element, about_us) = popup_ref.get().zip(about_us.get())?;
            let target_host = event.target()?;
            let target = target_host.dyn_ref::<Node>();
            show_popup.set(popup_element.contains(target) || about_us.contains(target));
            Some(())
        },
    );

    use_effect_with(
        (event_listener.setter(), callback),
        move |(event_listener, callback)| {
            let callback = callback.clone();
            let window = gloo::utils::window();
            let handle = EventListener::new(&window, "click", move |event| {
                if let Some(event) = event.dyn_ref::<web_sys::MouseEvent>() {
                    callback.emit(event.clone());
                }
            });
            event_listener.set(Some(handle));
        },
    );

    let on_start = props.on_run.clone();
    //  TODO: Replace Image from pause to step
    let on_step = props.on_step.clone();
    let on_stop = props.on_stop.clone();

    let author = gloo::utils::window()
        .location()
        .search()
        .expect("No search in URL!")
        .split('&')
        .find(|x| x.starts_with("?author="))
        .map(|x| x.replace("?author=", ""))
        .map(|x| x.replace("%20", " "));

    html! {
      <header>
          if *show_popup {
            <AboutPopup popup_ref={popup_ref.clone()}/>
          }
          <div class="header-left">
            <div class="logo">
              <a href="https://www.fiit.stuba.sk/" alt="FIIT">
                <img src="assets/logo_fiit.png" alt="FIIT logo" />
              </a>
            </div>
            <div class="code-author-container">
              if let Some(author) = author {
                  <div class="code-author">
                    {"Code Author: "}
                    <span class="code-author-name">{author}</span>
                  </div>
              }
            </div>
          </div>
          <div class="controls">
            <button onclick={move |_| on_start.emit(())} class="control-btn"><div class="start-btn"/></button>
            <button onclick={move |_| on_step.emit(())} class="control-btn"><div class="step-btn"/></button>
            <button onclick={move |_| on_stop.emit(())} class="control-btn"><div class="stop-btn"/></button>
          </div>
          <div class="help">
            <button
              onclick={move |_| show_popup.set(!*show_popup)}
              class="about-us"
              ref={popup_button_ref}
            >
              {"About Project"}
            </button>
          </div>
      </header>
    }
}
