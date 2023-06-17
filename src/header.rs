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

    use_effect_with_deps(move |_| {
        window_clone.set(Some(get_window().unwrap()));
    }, ());

    let on_start = props.on_run.clone();
    //  TODO: Replace Image to step
    let on_step = props.on_step.clone();
    let on_stop = props.on_stop.clone();
    let on_debug = props.on_debug.clone();

    let on_start = move |_| on_start.emit(());
    let on_step = move |_| on_step.emit(());
    let on_stop = move |_| on_stop.emit(());
    let on_debug = move |_| on_debug.emit(());

    let show_popup = use_state_eq(|| false);

    {
        let show_popup = show_popup.clone();
        let popup_ref = popup_ref.clone();
        let about_us_btn = popup_button_ref.clone();
        let popup_ref_d = popup_ref.clone();
        let about_us_btn_d = popup_button_ref.clone();

        use_event_listener(
            "click",
            move |event: web_sys::MouseEvent| {
                let (popup_element, about_us_btn) = match (popup_ref.get(), about_us_btn.get()) {
                    (Some(popup_element), Some(about_us_btn)) => (popup_element, about_us_btn),
                    _ => return,
                };
                let target_host = event.target().unwrap();
                let target = Some(target_host.unchecked_ref::<Node>());
                show_popup.set(popup_element.contains(target) || about_us_btn.contains(target));
            },
            (popup_ref_d, about_us_btn_d),
            (*window).clone().map(|w| w.into()),
        );
    }

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
            <button onclick={on_start} class="control-btn"><div class="compile-btn"/></button>
            <button onclick={on_step} class="control-btn"><div class="pause-btn"/></button>
            <button onclick={on_stop} class="control-btn"><div class="stop-btn"/></button>
            <button onclick={on_debug} class="control-btn"><div class="debug-btn"/></button>
          </div>
          <div class="help">
            <button
              onclick={move |_| { show_popup.set(!*show_popup) }}
              class="about-us"
              ref={popup_button_ref.clone()}
            >
              {"About Us"}
            </button>
          </div>
      </header>
    }
}