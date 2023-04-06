#![allow(non_camel_case_types)]

use monaco::{
  api::{CodeEditorOptions, TextModel},
  sys::editor::BuiltinTheme,
  yew::{CodeEditor, CodeEditorLink},
};
use yew::prelude::*;

const CONTENT: &str = r#"
  HALT
"#;

pub fn get_options() -> CodeEditorOptions {
  CodeEditorOptions::default()
    .with_language("ram".to_owned())
    .with_value(CONTENT.to_owned())
    .with_builtin_theme(BuiltinTheme::VsDark)
    .with_automatic_layout(true)
    .with_new_dimension(1000, 400)
}

#[derive(PartialEq, Properties)]
pub struct CustomEditorProps {
  pub on_editor_created: Callback<CodeEditorLink>,
  pub text_model: TextModel,
}

///
/// This is really just a helper component, so we can pass in props easier.
/// It makes it much easier to use, as we can pass in what we need, and it
/// will only re-render if the props change.
///
#[function_component(CustomEditor)]
pub fn custom_editor(props: &CustomEditorProps) -> Html {
  let CustomEditorProps {
    on_editor_created,
    text_model,
  } = props;

  html! {
      <CodeEditor classes={"full-height"} options={ get_options().to_sys_options() } {on_editor_created} model={text_model.clone()} />
  }
}
