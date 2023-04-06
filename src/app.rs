use crate::code_editor::CustomEditor;
use crate::io::custom_reader::CustomReader;
use crate::io::custom_writer::CustomWriter;
use std::cell::RefCell;
use std::io::BufReader;

use monaco::{api::TextModel, sys::editor::IStandaloneCodeEditor, yew::CodeEditorLink};
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use crate::io::input::InputComponent;
use crate::io::output::OutputComponent;
use ramemu::program::Program;
use ramemu::ram::Ram;

use wasm_bindgen::JsCast;

const INITIAL_CODE: &str = r#"
write =3
halt

"#;

#[function_component(App)]
pub fn app() -> Html {
  let text_model = use_state_eq(|| TextModel::create(INITIAL_CODE, Some("ram"), None).unwrap());
  let code = use_state_eq(|| String::from(INITIAL_CODE));
  let interpretator_output = use_state_eq(|| String::from(""));

  let stdout = use_state(|| String::from(""));

  let reader = use_memo(|_| RefCell::new(CustomReader::new()), ());
  let writer = use_memo(|_| Some(RefCell::new(CustomWriter::new())), ());

  let stdout_clone = stdout.clone();
  use_effect_with_deps(
    move |writer| {
      if let Some(writer) = &**writer {
        log::info!("Setting callback");
        let on_write = Callback::from(move |data: String| {
          stdout_clone.set((*stdout_clone).clone() + &data);
        });

        writer.borrow_mut().set_on_write(on_write);
      }
    },
    writer.clone(),
  );

  let iout = interpretator_output.clone();
  use_effect_with_deps(
    move |code| {
      let out = match Program::from_source(code) {
        Ok(program) => {
          let reader = BufReader::new(std::io::empty());
          if let Some(writer) = &*writer {
            let mut ram = Ram::new(program, Box::new(reader), Box::new(writer.borrow().clone()));
            format!("{:?}", ram.run())
          } else {
            format!("No writer {}", "!")
          }
        }
        Err(e) => format!("{:?}", e),
      };
      iout.set(out);
    },
    code.clone(),
  );

  let on_editor_created = {
    let text_model = text_model.clone();
    // let code = code.clone();

    let js_closure = {
      let text_model = text_model.clone();

      Closure::<dyn Fn()>::new(move || {
        code.set(text_model.get_value());
      })
    };

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

  let on_submit = {
    // let inputet = stdin.clone();
    // move |input: String| {
    //   inputet.set(input);
    // }
    |_input: String| {}
  };

  html! {
      <div id="code-wrapper">
          <div id="code-editor">
              <CustomEditor {on_editor_created} text_model={(*text_model).clone()} />
          </div>
          <OutputComponent output={AttrValue::from(stdout.to_string())} />
          <InputComponent {on_submit} reader={reader.clone()} input={"".to_string()} />
          <div id="event-log-wrapper">
              <div id="event-log">
                  <h2>{"Code (press CTRL+Enter / Command+Enter to view)"}</h2>
                  <pre> {interpretator_output.to_string()} </pre>
              </div>
          </div>
      </div>
  }
}
