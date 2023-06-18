use ramemu::errors::{InterpretError, ParseError};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum OutputComponentErrors {
    InterpretError(InterpretError),
    ParseError(ParseError),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub error: Option<OutputComponentErrors>,
    pub output: AttrValue,
}

#[function_component(OutputComponent)]
pub fn output_component(props: &Props) -> Html {
    let output = match &props.error {
        Some(OutputComponentErrors::InterpretError(err)) => format!("{:?}", err),
        Some(OutputComponentErrors::ParseError(err)) => format!("{:?}", err),
        None => props.output.to_string(),
    };
    html! {
      <div class="console-output">
        if props.error.is_some() {
          <span class="console-error-fg console-bold">{"Error: "}</span>
        }
        <span style="white-space:pre"> { output } </span>
      </div>
    }
}
