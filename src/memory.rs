use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
  entries: Vec<i64>,
}

#[function_component(Memory)]
pub fn memory() -> Html {
  html! {
    <div class="memory">
      <div class="memory__title">
        <h1 class="title is-1">{ "Memory" }</h1>
      </div>
      <div class="memory__content">
        <p>{ "This is the memory page" }</p>
      </div>
    </div>
  }
}
