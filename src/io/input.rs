#![allow(non_camel_case_types)]

use std::{cell::RefCell, rc::Rc};

use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::custom_reader::CustomReader;

pub struct InputComponent {
  on_submit: Callback<String>,
  input: String,
  reader: Rc<RefCell<CustomReader>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub on_submit: Callback<String>,
  pub reader: Rc<RefCell<CustomReader>>,
  pub input: String,
}

impl Component for InputComponent {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      on_submit: ctx.props().on_submit.clone(),
      input: ctx.props().input.clone(),
      reader: ctx.props().reader.clone(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::UpdateInput(input) => {
        self.input = input;
        true
      }
      Msg::Submit => {
        let input = std::mem::take(&mut self.input);
        self.reader.borrow_mut().set_input(input.clone());
        self.on_submit.emit(input);
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let onchange = ctx.link().batch_callback(|e: Event| {
      Some(Msg::UpdateInput(
        e.target_dyn_into::<HtmlInputElement>()?.value(),
      ))
    });
    html! {
      <>
        <input
          class="user-input"
          placeholder="Enter input"
          value={self.input.clone()}
          {onchange}
        />
        <button onclick={ctx.link().callback(|_| Msg::Submit)}>
          { "Submit" }
        </button>
      </>
    }
  }
}

pub enum Msg {
  UpdateInput(String),
  Submit,
}
