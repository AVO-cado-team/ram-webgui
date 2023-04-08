#![allow(non_camel_case_types)]

use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

pub struct InputComponent {
  on_change: Callback<String>,
  input: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub on_change: Callback<String>,
}

impl Component for InputComponent {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      on_change: ctx.props().on_change.clone(),
      input: String::new(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::UpdateInput(input) => {
        self.input = input;
        self.on_change.emit(self.input.clone());
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let onchange = ctx.link().batch_callback(|e: Event| {
      Some(Msg::UpdateInput(
        e.target_dyn_into::<HtmlTextAreaElement>()?.value(),
      ))
    });
    html! {
      <>
        <textarea
          class="user-input"
          placeholder="Enter input"
          value={self.input.clone()}
          {onchange}
        />
      </>
    }
  }
}

pub enum Msg {
  UpdateInput(String),
}
