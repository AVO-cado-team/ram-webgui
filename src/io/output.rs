use yew::prelude::*;

pub struct OutputComponent {
  pub output: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub output: AttrValue,
}

impl Component for OutputComponent {
  type Message = Msg;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    Self {
      output: ctx.props().output.to_string(),
    }
  }

  fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
    self.output = ctx.props().output.to_string();
    true
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
        <pre class="output">{ &self.output }</pre>
    }
  }
}

pub enum Msg {
  // AppendOutput(String),
}
