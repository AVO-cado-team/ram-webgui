use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub output: AttrValue,
}

#[function_component(OutputComponent)]
pub fn output_component(props: &Props) -> Html {
  html! {
    <pre class="output">{ &props.output }</pre>
  }
}
