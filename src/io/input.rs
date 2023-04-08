#![allow(non_camel_case_types)]

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub on_change: Callback<String>,
}

#[function_component(InputComponent)]
pub fn input_component(props: &Props) -> Html {
  let on_change = props.on_change.clone();

  let handle_change = move |event: InputEvent| {
    if let Some(input) = event.target_dyn_into::<HtmlTextAreaElement>() {
      log::info!("Input value: {}", input.value());
      on_change.emit(input.value());
    }
  };

  html! {
    <>
      <textarea
        class="user-input"
        placeholder="Enter input"
        oninput={handle_change}
      />
    </>
  }
}
