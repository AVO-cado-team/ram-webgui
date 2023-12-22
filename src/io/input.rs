#![allow(non_camel_case_types)]

use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Callback<String>,
}

const DEFAULT_STDIN: &str = r" 3 4 ";

#[function_component(InputComponent)]
pub fn input_component(props: &Props) -> Html {
    let on_change = props.on_change.clone();
    let value = use_state(|| String::new());

    let on_change2 = props.on_change.clone();
    let value_cloned = value.clone();

    use_effect_with((), move |_| {
        let value = LocalStorage::get("stdin").unwrap_or_else(|_| DEFAULT_STDIN.to_string());
        value_cloned.set(value.clone());
        on_change2.emit(value);
    });

    let value_cloned = value.clone();

    let handle_change = move |event: InputEvent| {
        let Some(input) = event.target_dyn_into::<HtmlInputElement>() else {
            log::error!("Failed to cast event target to HtmlTextAreaElement");
            return;
        };
        value_cloned.set(input.value());
        on_change.emit(input.value());
    };

    html! {
      <div class="console-input">
        <div class="input-marker">{">>>"}</div>
        <input
          type="text"
          class="input-values"
          placeholder="Enter input"
          oninput={handle_change}
          value={value.to_string()}
        />
      </div>
    }
}
