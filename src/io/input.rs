#![allow(non_camel_case_types)]

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub default_value: AttrValue,
}

#[function_component(InputComponent)]
pub fn input_component(props: &Props) -> Html {
    let on_change = props.on_change.clone();
    let value = use_state(|| props.default_value.to_string());
    let value_cloned = value.clone();

    let handle_change = move |event: InputEvent| {
        // It could be an If let, but I prefer to panic here
        let input = event
            .target_dyn_into::<HtmlInputElement>()
            .expect("Failed to cast event target to HtmlTextAreaElement");
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
