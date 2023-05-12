use monaco::{
  api::{CodeEditorOptions, TextModel},
  sys::editor::{IEditorOptionsTabCompletion, IStandaloneEditorConstructionOptions},
  // sys::editor::BuiltinTheme,
  yew::{CodeEditor, CodeEditorLink},
};
use yew::prelude::*;

use crate::monaco_ram::{ID, THEME};

pub fn get_options(value: String) -> IStandaloneEditorConstructionOptions {
  let options = CodeEditorOptions::default()
    .with_language(ID.to_owned())
    .with_theme(THEME.to_owned())
    .with_automatic_layout(true)
    .with_value(value)
    .to_sys_options();

  options.set_font_size(Some(16.0));
  options.set_tab_completion(Some(IEditorOptionsTabCompletion::On));

  options
}

#[derive(PartialEq, Properties)]
pub struct CustomEditorProps {
  pub on_editor_created: Callback<CodeEditorLink>,
  pub text_model: TextModel,
  pub value: AttrValue,
}

pub struct CustomEditor {}

impl Component for CustomEditor {
  type Message = ();
  type Properties = CustomEditorProps;

  fn create(_ctx: &Context<Self>) -> Self {
    monaco::workers::ensure_environment_set();

    Self {}
  }

  fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
    false
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let CustomEditorProps {
      on_editor_created,
      text_model,
      value,
    } = ctx.props();

    html! {
      <CodeEditor
        classes={"editor"}
        options={get_options(value.to_string())}
        {on_editor_created} model={text_model.clone()}
      />
    }
  }
}
