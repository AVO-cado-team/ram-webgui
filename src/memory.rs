use ramemu::registers::Registers;
use yew::prelude::*;

const WINDOW_LENGTH: usize = 100;
const STEP_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
  pub entries: Registers<i64>,
}

#[function_component(Memory)]
pub fn memory(props: &Props) -> Html {
  let registers = &props.entries;

  let starting_index = use_state(|| 0usize);

  let register_entries = (0..WINDOW_LENGTH)
    .map(|i| {
      let index = *starting_index + i;
      let value = registers.get(index);
      html! {
        <div class="register" key={index} >
          <div class="register-num"><p>{format!("R{}", index)}</p></div>
          <div class="register-val">{value.to_string()}</div>
        </div>
      }
    })
    .collect::<Html>();

  let _on_previous_click = {
    let starting_index = starting_index.clone();
    Callback::from(move |_: MouseEvent| {
      starting_index.set(starting_index.saturating_sub(STEP_SIZE));
    })
  };

  let _on_next_click = {
    Callback::from(move |_: MouseEvent| {
      starting_index.set(starting_index.saturating_add(STEP_SIZE));
    })
  };

  html! {
      <div class="registers-container">
        // <button onclick={on_previous_click}>{"Previous"}</button>
        // <button onclick={on_next_click}>{"Next"}</button>
        {register_entries}
      </div>
  }
}
