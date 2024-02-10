use wasm_bindgen::{prelude::*, JsValue};

#[cfg(not(feature = "ssr"))]
use js_sys::{Array, Object};
#[cfg(not(feature = "ssr"))]
use web_sys::HtmlAnchorElement;

#[cfg(not(feature = "ssr"))]
use monaco::{
    api::{CodeEditor, TextModel},
    sys::{
        editor::{ICodeEditor, IEditor, IIdentifiedSingleEditOperation, ITextModel},
        Position, Range,
    },
};

use yew::prelude::*;

#[function_component]
pub fn HydrationGate(props: &Props) -> Html {
    let is_hydrated = use_state(|| false);
    let is_hydrated_cloned = is_hydrated.clone();

    use_effect_with((), move |_| is_hydrated_cloned.set(true));

    if *is_hydrated {
        html! { for props.children.iter() }
    } else {
        props.placeholder.clone()
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub placeholder: Html,
}

#[cfg(not(feature = "ssr"))]
fn get_selection_or_cursor_range(ieditor: &ICodeEditor) -> Option<Range> {
    let selection = ieditor.get_selection()?;

    let start_line = selection.start_line_number();
    let end_line = selection.end_line_number();
    let start_column = selection.start_column();
    let end_column = selection.end_column();

    if start_line as u64 == end_line as u64 && start_column as u64 == end_column as u64 {
        // Treat as cursor position, not selection
        let cursor_line = ieditor.get_position()?.line_number();
        let max_column = ieditor.get_model()?.get_line_max_column(cursor_line);
        Some(Range::new(cursor_line, 0., cursor_line, max_column))
    } else {
        // Handle normal selection
        Some(Range::new(start_line, start_column, end_line, end_column))
    }
}

#[cfg(not(feature = "ssr"))]
pub fn comment_code(editor: &CodeEditor, model: &TextModel) -> Result<(), ()> {
    let ieditor: &ICodeEditor = editor.as_ref();
    let range = get_selection_or_cursor_range(ieditor).ok_or(())?;

    let itext_model: &ITextModel = model.as_ref();
    let text = itext_model.get_value_in_range(&range.clone().unchecked_into(), None);

    let (new_text, do_comment) = {
        match text
            .lines()
            .map(|l| l.strip_prefix('#'))
            .collect::<Option<Vec<_>>>()
        {
            Some(lines) => (lines.join("\n"), false),
            None => {
                let lines: Vec<_> = text.lines().map(|l| format!("#{l}")).collect();
                (lines.join("\n"), true)
            }
        }
    };

    let edits = Array::new();
    let edit: IIdentifiedSingleEditOperation = Object::new().unchecked_into();
    edit.set_range(&range);
    edit.set_text(Some(&new_text));
    edits.push(&edit);

    let line_number = range.start_line_number();
    let column = ieditor.get_position().ok_or(())?.column();

    ieditor.execute_edits("comment", &edits, None);

    // Clear the selection after single line comment
    if range.start_line_number() as u64 == range.end_line_number() as u64
        && range.start_column() == 0.0
    {
        let ieditor: &IEditor = editor.as_ref();
        let column = column + if do_comment { 1.0 } else { -1.0 };
        let column = column.max(1.0);
        let new_position = Position::new(line_number, column);
        ieditor.set_position(new_position.unchecked_ref());
    };
    Ok(())
}

pub fn get_author() -> Option<String> {
    gloo::utils::window()
        .location()
        .search()
        .unwrap_or_default()
        .replace('?', "")
        .split('&')
        .find_map(|x| x.strip_prefix("author="))
        .map(|x| x.replace('+', "%20"))
        .and_then(|x| Some(urlencoding::decode(&x).ok()?.into_owned()))
        .and_then(|x| {
            let res: String = x
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == ',' || *c == '.' || *c == ' ')
                .collect();
            (!res.is_empty()).then_some(res)
        })
}

#[cfg(not(feature = "ssr"))]
pub fn download_code(content: &str) -> Result<(), JsValue> {
    let document = gloo::utils::document();
    let body = gloo::utils::body();

    let element = document
        .create_element("a")?
        .dyn_into::<HtmlAnchorElement>()?;

    let href = format!(
        "data:text/plain;charset=utf-8,{}",
        urlencoding::encode(content)
    );
    element.set_href(&href);
    element.set_download("project.ram");
    element.style().set_property("display", "none")?;

    body.append_child(&element)?;
    element.click();
    body.remove_child(&element)?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn clipboard_callback(success: bool) {
    use crate::store::dispatch;
    use std::time::Duration;
    use yew::platform::time::sleep;

    dispatch().reduce_mut(|store| store.copy_button_state = Some(success));
    yew::platform::spawn_local(async {
        sleep(Duration::from_secs(3)).await;
        dispatch().reduce_mut(|store| store.copy_button_state = None)
    })
}

#[cfg(not(feature = "ssr"))]
pub fn copy_to_clipboard(text: &str) {
    thread_local! {
        static CLOSURE: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(|v| {
            clipboard_callback(v.as_bool().unwrap_or_default());
        }));
    }

    CLOSURE.with(|c| copy_to_clipboard_js(text, c))
}

#[wasm_bindgen(module = "/js/copyToClipboard.js")]
extern "C" {
    #[wasm_bindgen(js_name = "copyToClipboard")]
    fn copy_to_clipboard_js(text: &str, callback: &Closure<dyn FnMut(JsValue)>);
}
