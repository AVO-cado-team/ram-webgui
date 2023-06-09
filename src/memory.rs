use ramemu::registers::RegisterId;
use ramemu::registers::Registers;
use yew::prelude::*;

const WINDOW_LENGTH: usize = 100;
const STEP_SIZE: usize = 50;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub entries: Registers<i64>,
}

#[function_component(Memory)]
pub fn memory(props: &Props) -> Html {
    let registers = &props.entries;

    let starting_index = use_state(|| 0);

    let register_entries = (0..WINDOW_LENGTH)
        .map(|i| {
            let index = *starting_index + i;
            let value = registers.get(RegisterId(index));
            let mut class = "register".to_string();
            if index == 0 {
                class += " acc";
            }
            html! {
              <div class={class} key={index} >
                <div class="register-num"><p>{format!("{}", index)}</p></div>
                <div class="register-val">{value.to_string()}</div>
              </div>
            }
        })
        .collect::<Html>();

    #[allow(unused_variables)]
    let on_previous_click = Callback::from({
        let starting_index = starting_index.clone();
        move |_: MouseEvent| starting_index.set(starting_index.saturating_sub(STEP_SIZE))
    });

    #[allow(unused_variables)]
    let on_next_click = Callback::from({
        let starting_index = starting_index;
        move |_: MouseEvent| starting_index.set(starting_index.saturating_add(STEP_SIZE))
    });

    html! {
        <div class="registers-container">
          // <button onclick={on_previous_click}>{"Previous"}</button>
          // <button onclick={on_next_click}>{"Next"}</button>
          <div class="register acc">
            <div class="register-num"><p>{"R"}</p></div>
            <div class="register-val">{"Value"}</div>
          </div>
          {register_entries}
        </div>
    }
}
