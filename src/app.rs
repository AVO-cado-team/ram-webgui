#![allow(non_camel_case_types)]

use std::io::BufReader;

use monaco::{
  api::{CodeEditorOptions, TextModel},
  sys::editor::{BuiltinTheme, IStandaloneCodeEditor},
  yew::{CodeEditor, CodeEditorLink},
};
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use ramemu::program::Program;
use ramemu::ram::Ram;

use wasm_bindgen::JsCast;

const CONTENT: &str = r#"
  HALT
"#;

fn get_options() -> CodeEditorOptions {
  CodeEditorOptions::default()
    .with_language("ram".to_owned())
    .with_value(CONTENT.to_owned())
    .with_builtin_theme(BuiltinTheme::VsDark)
    .with_automatic_layout(true)
    .with_new_dimension(1000, 400)
}

#[derive(PartialEq, Properties)]
pub struct CustomEditorProps {
  on_editor_created: Callback<CodeEditorLink>,
  text_model: TextModel,
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

#[function_component(App)]
pub fn app() -> Html {
  let text_model = use_state_eq(|| TextModel::create(CONTENT, Some("ram"), None).unwrap());

  let code = use_state_eq(|| String::from(CONTENT));

  let on_editor_created = {
    let text_model = text_model.clone();
    let code = code.clone();

    let js_closure = {
      let text_model = text_model.clone();

      Closure::<dyn Fn()>::new(move || {
        let program = Program::from_source(&text_model.get_value());
        if let Ok(program) = program {
          let reader = BufReader::new(std::io::empty());
          let writer = std::io::sink();
          let mut ram = Ram::new(program, Box::new(reader), Box::new(writer));
          code.set(format!("{:?}", ram.run()));
        }
        // code.set(text_model.get_value());
      })
    };

    // Here we define our callback, we use use_callback as we want to re-render when dependencies change.
    // See https://yew.rs/docs/concepts/function-components/state#general-view-of-how-to-store-state
    use_callback(
      move |editor_link: CodeEditorLink, _text_model| {
        editor_link.with_editor(|editor| {
          // Registers Ctrl/Cmd + Enter hotkey
          let keycode =
            monaco::sys::KeyCode::Enter.to_value() | (monaco::sys::KeyMod::ctrl_cmd() as u32);
          let raw_editor: &IStandaloneCodeEditor = editor.as_ref();

          raw_editor.add_command(keycode.into(), js_closure.as_ref().unchecked_ref(), None);
        });
      },
      text_model,
    )
  };
  html! {
      <div id="code-wrapper">
          <div id="code-editor">
              <CustomEditor {on_editor_created} text_model={(*text_model).clone()} />
          </div>
          <div id="event-log-wrapper">
              <div id="event-log">
                  <h2>{"Code (press CTRL+Enter / Command+Enter to view)"}</h2>
                  <pre>{code.to_string()}</pre>
              </div>
          </div>
      </div>
  }
}
