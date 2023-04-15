#![allow(non_camel_case_types)]
use yew::prelude::*;

use crate::about_popup::AboutPopup;

#[derive(Default, Clone, PartialEq, Properties)]
pub struct Props {
  pub on_start: Callback<()>,
  pub on_pause: Callback<()>,
  pub on_stop: Callback<()>,
  pub on_debug: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
  let on_start = props.on_start.clone();
  let on_pause = props.on_pause.clone();
  let on_stop = props.on_stop.clone();
  let on_debug = props.on_debug.clone();

  let on_start = move |_| on_start.emit(());
  let on_pause = move |_| on_pause.emit(());
  let on_stop = move |_| on_stop.emit(());
  let on_debug = move |_| on_debug.emit(());

  let show_popup = use_state(|| false);

  html! {
    <header>
        if *show_popup {
          <AboutPopup />
        }
        <div class="logo">
          <a href="https://www.fiit.stuba.sk/" alt="FIIT">
            <img src="assets/logo_fiit.png" alt="FIIT logo" />
          </a>
        </div>
        <div class="controls">
          <button onclick={on_start} class="control-btn"><div class="compile-btn"/></button>
          <button onclick={on_pause} class="control-btn"><div class="pause-btn"/></button>
          <button onclick={on_stop} class="control-btn"><div class="stop-btn"/></button>
          <button onclick={on_debug} class="control-btn"><div class="debug-btn"/></button>
        </div>
        <div class="help">
          <button
            onclick={move |_| { show_popup.set(!*show_popup) }}
            class="about-us"
          >
            {"About Us"}
          </button>
        </div>
    </header>
  }
}
