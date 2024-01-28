use std::rc::Rc;

use js_sys::{Array, Object};
use monaco::{
    api::{CodeEditor, DisposableClosure},
    sys::{
        editor::{
            self, IEditorMouseEvent, IModelDecorationOptions, IModelDeltaDecoration,
            IStandaloneCodeEditor, MouseTargetType,
        },
        IRange, Range,
    },
};
use wasm_bindgen::{JsCast, JsValue};
use yewdux::{Dispatch, Listener};

use crate::store::Store;

#[derive(Default)]
pub struct EditorStoreListener {
    old: std::rc::Rc<Store>,
    error_decoration_ids: Array,
    breakpoint_decoration_ids: Array,
    debug_line_decoration_ids: Array,
}
impl Listener for EditorStoreListener {
    type Store = Store;

    fn on_change(&mut self, _: &yewdux::Context, state: std::rc::Rc<Self::Store>) {
        log::info!("EditorStoreListener::on_change {:?}", state.error.as_ref());
        let editor = state.editor.clone();
        editor.with_editor(|editor| {
            // clear all errors
            clean_errors(editor, &self.error_decoration_ids);
            clean_breakpoints(editor, &self.breakpoint_decoration_ids);
            clean_debug_line(editor, &self.debug_line_decoration_ids);
            self.error_decoration_ids = js_sys::Array::new();
            self.breakpoint_decoration_ids = js_sys::Array::new();
            self.debug_line_decoration_ids = js_sys::Array::new();

            if let Some(error) = state.error.as_ref() {
                self.error_decoration_ids = draw_error(editor, error);
            }
            for line in state.breakpoints.iter() {
                let breakpoint_decoration_id = draw_breakpoint(editor, *line);
                for id in breakpoint_decoration_id.iter() {
                    self.breakpoint_decoration_ids.push(&id);
                }
            }
            if state.current_debug_line != 0 {
                self.debug_line_decoration_ids = draw_debug_line(editor, state.current_debug_line);
            }
        });
        self.old = state.clone();
    }
}

pub fn setup_breakpoints(editor: &CodeEditor) -> DisposableClosure<dyn FnMut(IEditorMouseEvent)> {
    editor.on_mouse_down(|e| {
        if let MouseTargetType::GutterLineNumbers = e.target().type_() {
            let Some(line_number) = e.target().position() else {
                return;
            };
            let line_number = line_number.line_number() as usize;
            // add / remove breakpoint on clicked position
            let store: Rc<Store> = Dispatch::global().get();
            if store.breakpoints.contains(&line_number) {
                Dispatch::global().reduce_mut(|s: &mut Store| {
                    s.breakpoints.remove(&line_number);
                });
            } else {
                Dispatch::global().reduce_mut(|s: &mut Store| {
                    s.breakpoints.insert(line_number);
                });
            }
            let store: Rc<Store> = Dispatch::global().get();
            log::info!("Breakpoints: {:?}", store.breakpoints);
        }
    })
}

fn clean_errors(editor: &monaco::api::CodeEditor, decoration_ids: &Array) {
    let empty = js_sys::Array::new();
    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
    raw_editor.delta_decorations(decoration_ids, &empty);
}

fn draw_error(
    editor: &monaco::api::CodeEditor,
    error: &crate::io::output::OutputComponentErrors,
) -> Array {
    let (kind, line, error_classname) = match &error {
        crate::io::output::OutputComponentErrors::InterpretError(e) => (
            format!("{:?}", e.kind),
            e.line,
            "runtime-error-line-highlight",
        ),
        crate::io::output::OutputComponentErrors::ParseError(e) => (
            format!("{:?}", e.kind),
            e.line,
            "syntax-error-line-highlight",
        ),
    };
    let line = line as f64;

    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
    let empty = js_sys::Array::new();
    let new_decorations = js_sys::Array::new();

    let options = Object::new().unchecked_into::<IModelDecorationOptions>();
    options.set_class_name(Some(error_classname));
    options.set_is_whole_line(Some(true));
    options.set_hover_message(&JsValue::from_str(kind.as_str()));
    options.set_glyph_margin_hover_message(&JsValue::from_str(kind.as_str()));
    options.set_glyph_margin_class_name(Some("error-glyph-margin"));

    let range: Range = Range::new(line, 1., line, 3.);
    let irange: IRange = range.unchecked_into();

    let decoration = Object::new().unchecked_into::<IModelDeltaDecoration>();
    decoration.set_options(&options);
    decoration.set_range(&irange);

    new_decorations.push(&decoration);

    raw_editor.delta_decorations(&empty, &new_decorations)
}

fn clean_breakpoints(editor: &monaco::api::CodeEditor, decoration_ids: &Array) {
    let empty = js_sys::Array::new();
    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
    raw_editor.delta_decorations(decoration_ids, &empty);
}

fn draw_breakpoint(editor: &monaco::api::CodeEditor, line: usize) -> Array {
    let line = line as f64;

    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
    let empty = js_sys::Array::new();
    let new_decorations = js_sys::Array::new();

    let options = Object::new().unchecked_into::<IModelDecorationOptions>();
    options.set_glyph_margin_class_name(Some("breakpoint"));
    options.set_is_whole_line(Some(true));

    let range: Range = Range::new(line, 1., line, 3.);
    let irange: IRange = range.unchecked_into();

    let decoration = Object::new().unchecked_into::<IModelDeltaDecoration>();
    decoration.set_options(&options);
    decoration.set_range(&irange);

    new_decorations.push(&decoration);

    raw_editor.delta_decorations(&empty, &new_decorations)
}

fn clean_debug_line(editor: &monaco::api::CodeEditor, decoration_ids: &Array) {
    let empty = js_sys::Array::new();
    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
    raw_editor.delta_decorations(decoration_ids, &empty);
}

fn draw_debug_line(editor: &monaco::api::CodeEditor, line: usize) -> Array {
    let line = line as f64;

    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
    let empty = js_sys::Array::new();
    let new_decorations = js_sys::Array::new();

    let options = Object::new().unchecked_into::<IModelDecorationOptions>();
    options.set_is_whole_line(Some(true));
    options.set_class_name(Some("debug-line-highlight"));

    let range: Range = Range::new(line, 1., line, 3.);
    let irange: IRange = range.unchecked_into();

    let decoration = Object::new().unchecked_into::<IModelDeltaDecoration>();
    decoration.set_options(&options);
    decoration.set_range(&irange);

    new_decorations.push(&decoration);

    raw_editor.delta_decorations(&empty, &new_decorations)
}
