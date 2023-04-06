use crate::code_editor::CustomEditor;
use std::io::BufReader;

use monaco::{api::TextModel, sys::editor::IStandaloneCodeEditor, yew::CodeEditorLink};
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use ramemu::program::Program;
use ramemu::ram::Ram;

const CONTENT: &str = r#"
  HALT
"#;

use wasm_bindgen::JsCast;

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
