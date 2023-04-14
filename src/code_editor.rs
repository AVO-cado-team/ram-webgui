#![allow(non_camel_case_types)]

use monaco::{
  api::{CodeEditorOptions, TextModel},
  // sys::editor::BuiltinTheme,
  yew::{CodeEditor, CodeEditorLink},
};
use yew::prelude::*;

const KEYWORDS: [&str; 18] = [
  "ADD", "SUB", "MUL", "MULT", "DIV", "JUMP", "JMP", "JZ", "JZERO", "JGZ", "JGTZ", "LOAD", "STORE",
  "INPUT", "READ", "WRITE", "OUTPUT", "HALT",
];

pub fn get_options(value: String) -> CodeEditorOptions {
  CodeEditorOptions::default()
    .with_language("ram".to_owned())
    .with_theme("ram-theme".to_owned())
    // .with_builtin_theme(BuiltinTheme::VsDark)
    .with_automatic_layout(true)
    .with_new_dimension(1000, 400)
    .with_value(value)
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
      options={get_options(value.to_string()).to_sys_options()}
      {on_editor_created} model={text_model.clone()}
    />
  }
}
