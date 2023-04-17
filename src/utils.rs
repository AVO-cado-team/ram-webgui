use js_sys::{Array, Object};
use monaco::api::{CodeEditor, TextModel};
use monaco::sys::editor::{ICodeEditor, IIdentifiedSingleEditOperation, ITextModel};
use monaco::sys::Range;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, convert::FromWasmAbi, JsCast};
use web_sys::{Document, Event, EventTarget, HtmlAnchorElement, HtmlElement};
use yew::prelude::*;

pub fn get_from_local_storage(key: &str) -> Option<String> {
  let window = web_sys::window()?;
  let storage = window.local_storage().ok()??;
  storage.get_item(key).unwrap_or(None)
}

pub fn save_code_to_storage(key: &str, value: &str) {
  let window = web_sys::window().expect("no global `window` exists");
  let storage = window
    .local_storage()
    .expect("failed to access local storage")
    .expect("failed to access local storage");
  storage.set_item(key, value).unwrap();
}

#[hook]
pub fn use_event_listener<F, T, E>(event: &'static str, handler: F, deps: T, element: EventTarget)
where
  F: Fn(E) + 'static,
  T: PartialEq + 'static,
  E: JsCast + FromWasmAbi + 'static,
{
  log::info!("here");
  use_effect_with_deps(
    move |_: &T| {
      let closure = Closure::wrap(Box::new(move |event: Event| {
        if let Ok(event) = event.dyn_into::<E>() {
          handler(event);
        }
      }) as Box<dyn FnMut(Event)>);

      element
        .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
        .expect("Failed to add event listener");

      move || {
        element
          .remove_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
          .expect("Failed to add event listener");
      }
    },
    deps,
  );
}

pub fn comment_selected_code(editor: &CodeEditor, model: &TextModel) {
  let ieditor: &ICodeEditor = editor.as_ref();
  if let Some(selection) = ieditor.get_selection() {
    let range = Range::new(
      selection.start_line_number(),
      selection.start_column(),
      selection.end_line_number(),
      selection.end_column(),
    );

    let itext_model: &ITextModel = model.as_ref();
    let text = itext_model.get_value_in_range(&range.clone().unchecked_into(), None);

    let lines: Vec<&str> = text.lines().collect();
    let comment = lines
      .iter()
      .map(|line| format!("#{}", line))
      .collect::<Vec<String>>()
      .join("\n");

    let edits = Array::new();
    let edit = Object::new().unchecked_into::<IIdentifiedSingleEditOperation>();
    edit.set_range(&range);
    edit.set_text(Some(&comment));
    edits.push(&edit);

    ieditor.execute_edits("comment", &edits, None);
  }
}

pub fn download_code(content: &str) -> Result<(), ()> {
  let window = web_sys::window().ok_or(())?;
  let document: Document = window.document().ok_or(())?;
  let body = document.body().ok_or(())?;

  let element = document
    .create_element("a")
    .map_err(|_| ())?
    .dyn_into::<HtmlAnchorElement>()
    .map_err(|_| ())?;

  let href = format!(
    "data:text/plain;charset=utf-8,{}",
    urlencoding::encode(content)
  );
  element.set_href(&href);
  element.set_download("project.ram");
  element
    .style()
    .set_property("display", "none")
    .map_err(|_| ())?;

  body.append_child(&element).map_err(|_| ())?;
  element.click();
  body.remove_child(&element).map_err(|_| ())?;
  Ok(())
}

// This shit should not go to production
#[allow(dead_code)]
pub fn show_content() -> Result<(), ()> {
  let window = web_sys::window().ok_or(())?;
  let document: Document = window.document().ok_or(())?;
  let main = document.get_element_by_id("ram-web").ok_or(())?;
  let loader = document.get_element_by_id("loader").ok_or(())?;

  let counter = Rc::new(RefCell::new(0.0));

  let closure = {
    let counter = Rc::clone(&counter);
    let main = main.dyn_into::<HtmlElement>().map_err(|_| ())?;
    let loader = loader.dyn_into::<HtmlElement>().map_err(|_| ())?;

    Closure::wrap(Box::new(move || {
      let mut counter_value = counter.borrow_mut();
      if *counter_value >= 1.0 {
        let _ = main.style().set_property("opacity", "1");
        let _ = loader.style().set_property("display", "none");
      } else {
        *counter_value += 0.1;
        let _ = main
          .style()
          .set_property("opacity", &counter_value.to_string());
      }
    }) as Box<dyn FnMut()>)
  };

  window
    .set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 50)
    .map_err(|_| ())?;
  closure.forget(); // We don't want to drop the closure
  Ok(())
}
