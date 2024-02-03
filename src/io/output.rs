use std::fmt::format;

use ramemu::errors::{InterpretError, ParseError};
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum OutputComponentErrors {
    InterpretError(InterpretError),
    ParseError(ParseError),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub errors: Vec<OutputComponentErrors>,
    pub output: AttrValue,
}

#[function_component(OutputComponent)]
pub fn output_component(props: &Props) -> Html {
    let errors = props.errors.iter().map(|err| match err {
        // TODO:
        OutputComponentErrors::InterpretError(err) => {
            html! { <div class="console-runtime-error-fg console-bold">{format!("{err:?}")}</div> }
        }
        OutputComponentErrors::ParseError(err) => {
            html! { <div class="console-parse-error-fg console-bold">{format!("{err:?}")}</div> }
        }
    });
    html! {
      <div class="console-output">
        { for errors }
        <span style="white-space:pre"> { &props.output } </span>
      </div>
    }
}
