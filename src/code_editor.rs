#![allow(non_camel_case_types)]

use monaco::{
  api::{CodeEditorOptions, TextModel},
  sys::editor::{IStandaloneEditorConstructionOptions, IEditorOptionsTabCompletion},
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
    .with_new_dimension(1000, 400)
    .with_value(value)
    .to_sys_options();

  options.set_font_size(Some(20.0));
  options.set_tab_completion(Some(IEditorOptionsTabCompletion::On));
  options.set_font_family(Some("Droid Sans Mono"));

  options
}

#[derive(PartialEq, Properties)]
pub struct CustomEditorProps {
  pub on_editor_created: Callback<CodeEditorLink>,
  pub text_model: TextModel,
  pub value: AttrValue,
}

#[function_component(CustomEditor)]
pub fn custom_editor(props: &CustomEditorProps) -> Html {
  let CustomEditorProps {
    on_editor_created,
    text_model,
    value,
  } = props;

  monaco::workers::ensure_environment_set();

  html! {
    <CodeEditor
      classes={"full-height"}
      options={get_options(value.to_string())}
      {on_editor_created} model={text_model.clone()}
    />
  }
}
