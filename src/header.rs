use crate::utils::use_event_listener;
use wasm_bindgen::JsCast;
use web_sys::{window as get_window, Node, Window};
use yew::prelude::*;

use crate::about_popup::AboutPopup;

#[derive(Default, Clone, PartialEq, Properties)]
pub struct Props {
    pub on_run: Callback<()>,
    pub on_step: Callback<()>,
    pub on_stop: Callback<()>,
    pub on_debug: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let popup_ref = use_node_ref();
    let popup_button_ref = use_node_ref();
    let window = use_state(|| None::<Window>);
    let window_clone = window.clone();

    use_effect_with_deps(move |_| window_clone.set(Some(get_window().unwrap())), ());

    let on_start = props.on_run.clone();
    //  TODO: Replace Image from pause to step
    let on_step = props.on_step.clone();
    let on_stop = props.on_stop.clone();
    let on_debug = props.on_debug.clone();

    let show_popup = use_state_eq(|| false);

    use_event_listener(
        "click",
        {
            let show_popup = show_popup.clone();
            let popup_ref = popup_ref.clone();
            let about_us = popup_button_ref.clone();

            move |event: web_sys::MouseEvent| {
                if let Some((popup_element, about_us)) = popup_ref.get().zip(about_us.get()) {
                    let target_host = event.target().unwrap();
                    let target = Some(target_host.unchecked_ref::<Node>());
                    show_popup.set(popup_element.contains(target) || about_us.contains(target));
                };
            }
        },
        (popup_ref.clone(), popup_button_ref.clone()),
        (*window).clone().map(|w| w.into()),
    );

    html! {
      <header>
          if *show_popup {
            <AboutPopup popup_ref={popup_ref.clone()}/>
          }
          <div class="logo">
            <a href="https://www.fiit.stuba.sk/" alt="FIIT">
              <img src="assets/logo_fiit.png" alt="FIIT logo" />
            </a>
          </div>
          <div class="controls">
            <button onclick={move |_| on_start.emit(())} class="control-btn"><div class="compile-btn"/></button>
            <button onclick={move |_| on_step.emit(())} class="control-btn"><div class="pause-btn"/></button>
            <button onclick={move |_| on_stop.emit(())} class="control-btn"><div class="stop-btn"/></button>
            <button onclick={move |_| on_debug.emit(())} class="control-btn"><div class="debug-btn"/></button>
          </div>
          <div class="help">
            <button
              onclick={move |_| show_popup.set(!*show_popup)}
              class="about-us"
              ref={popup_button_ref.clone()}
            >
              {"About Us"}
            </button>
          </div>
      </header>
    }
}
