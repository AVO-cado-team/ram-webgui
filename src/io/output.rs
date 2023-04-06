use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct OutputProps {
  pub output: AttrValue,
}

#[function_component(OutputComponent)]
pub fn output_component(props: &OutputProps) -> Html {
  html! {
    <pre class="output">{ &props.output }</pre>
  }
}
