#![allow(non_camel_case_types)]

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[function_component(InputComponent)]
pub fn input_component() -> Html {
    let value = use_selector(|s: &Store| s.stdin.clone());

    let handle_change = |event: InputEvent| {
        let Some(input) = event.target_dyn_into::<HtmlInputElement>() else {
            log::error!("Failed to cast event target to HtmlTextAreaElement");
            return;
        };
        let value = input.value();
        Dispatch::global().reduce_mut(|s: &mut Store| s.stdin = value);
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
