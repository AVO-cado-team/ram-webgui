pub mod after_hydration;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use js_sys::{Array, Object};
use wasm_bindgen::{closure::Closure, convert::FromWasmAbi, JsCast};
use web_sys::{Document, Event, EventTarget, HtmlAnchorElement, HtmlElement};
use yew::prelude::*;

use monaco::api::{CodeEditor, TextModel};
use monaco::sys::editor::{ICodeEditor, IEditor, IIdentifiedSingleEditOperation, ITextModel};
use monaco::sys::{Position, Range};

pub fn get_from_local_storage(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(key).unwrap_or(None)
}

pub fn save_to_local_storage(key: &str, value: &str) {
    let _len = value.len();
    let window = web_sys::window().expect("no global `window` exists");
    let storage = window
        .local_storage()
        .expect("failed to access local storage");
    if let Some(storage) = storage {
        storage
            .set_item(key, value)
            .expect("failed to write to local storage");
    }
}

pub async fn sleep(delay: Duration) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay.as_millis() as i32);
    };

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

#[hook]
pub fn use_event_listener<F, T, E>(
    event: &'static str,
    handler: F,
    deps: T,
    element: Option<EventTarget>,
) where
    F: Fn(E) + 'static,
    T: PartialEq + 'static,
    E: JsCast + FromWasmAbi + 'static,
{
    use_effect_with_deps(
        move |(_, element): &(T, Option<EventTarget>)| {
            let closure = Closure::wrap(Box::new(move |event: Event| {
                let event = event
                    .dyn_into::<E>()
                    .expect("Failed to cast event into closure's argument");
                handler(event);
            }) as Box<dyn FnMut(Event)>);

            if let Some(element) = element {
                element
                    .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
                    .expect("Failed to add event listener");
            }
            let element = element.clone();
            move || {
                if let Some(element) = element {
                    element
                        .remove_event_listener_with_callback(
                            event,
                            closure.as_ref().unchecked_ref(),
                        )
                        .expect("Failed to remove event listener");
                }
            }
        },
        (deps, element),
    );
}

fn get_selection_or_cursor_range(ieditor: &ICodeEditor) -> Option<Range> {
    let selection = ieditor.get_selection()?;

    let start_line = selection.start_line_number();
    let end_line = selection.end_line_number();
    let start_column = selection.start_column();
    let end_column = selection.end_column();

    if start_line == end_line && start_column == end_column {
        // Treat as cursor position, not selection
        let cursor_line = ieditor.get_position()?.line_number();
        let max_column = ieditor.get_model()?.get_line_max_column(cursor_line);
        Some(Range::new(cursor_line, 0., cursor_line, max_column))
    } else {
        // Handle normal selection
        Some(Range::new(start_line, start_column, end_line, end_column))
    }
}

fn prepare_new_text(lines: Vec<&str>) -> (String, bool) {
    type callback = Box<dyn Fn(&str) -> String>;
    let (toggler, do_comment): (callback, bool) = if lines.iter().all(|l| l.starts_with('#')) {
        (Box::new(|line: &str| String::from(&line[1..])), false)
    } else {
        (Box::new(|line: &str| format!("#{}", line)), true)
    };

    let new_text = lines
        .into_iter()
        .map(|line| toggler(line))
        .collect::<Vec<String>>()
        .join("\n");
    (new_text, do_comment)
}

pub fn comment_code(editor: &CodeEditor, model: &TextModel) -> Result<(), ()> {
    let ieditor: &ICodeEditor = editor.as_ref();
    let range = get_selection_or_cursor_range(ieditor).ok_or(())?;

    let itext_model: &ITextModel = model.as_ref();
    let text = itext_model.get_value_in_range(&range.clone().unchecked_into(), None);
    let lines: Vec<&str> = text.lines().collect();
    let (new_text, do_comment) = prepare_new_text(lines);

    let edits = Array::new();
    let edit = Object::new().unchecked_into::<IIdentifiedSingleEditOperation>();
    edit.set_range(&range);
    edit.set_text(Some(&new_text));
    edits.push(&edit);

    let line_number = range.start_line_number();
    let column = ieditor.get_position().ok_or(())?.column();

    ieditor.execute_edits("comment", &edits, None);

    // Clear the selection after single line comment
    if range.start_line_number() == range.end_line_number() && range.start_column() == 0.0 {
        let ieditor: &IEditor = editor.as_ref();
        let column = column + if do_comment { 1.0 } else { -1.0 };
        let column = column.max(1.0);
        let new_position = Position::new(line_number, column);
        let iposition = <Position as AsRef<Position>>::as_ref(&new_position);
        ieditor.set_position(iposition.unchecked_ref());
    };
    Ok(())
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
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            50,
        )
        .map_err(|_| ())?;
    closure.forget(); // We don't want to drop the closure
    Ok(())
}
