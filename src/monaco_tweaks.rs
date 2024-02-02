use std::rc::Rc;

use js_sys::{Array, Object};
use monaco::{
    api::{CodeEditor, DisposableClosure},
    sys::editor::{
        IEditorMouseEvent, IModelDecorationOptions, IModelDeltaDecoration, IStandaloneCodeEditor,
        MouseTargetType,
    },
    sys::{IRange, Range},
};
use wasm_bindgen::{JsCast, JsValue};
use yewdux::Listener;

use crate::{
    io::output::OutputComponentErrors,
    store::{dispatch, Store},
};

#[derive(Default)]
pub struct EditorStoreListener {
    error_ids: Array,
    breakpoint_ids: Array,
    debug_line_ids: Array,
}

impl Listener for EditorStoreListener {
    type Store = Store;

    fn on_change(&mut self, _: &yewdux::Context, state: Rc<Self::Store>) {
        state.editor.with_editor(|editor| {
            let error_ids = state.error.as_ref().map_or(Array::new(), draw_error);

            let breakpoint_ids = Array::new();
            for &line in &state.breakpoints {
                let breakpoint_decoration_id = breakpoints_dec(line as f64);
                for id in breakpoint_decoration_id.iter() {
                    breakpoint_ids.push(&id);
                }
            }

            let debug_line_ids = if state.current_debug_line != 0 {
                debug_line_dec(state.current_debug_line as f64)
            } else {
                Array::new()
            };

            let editor: &IStandaloneCodeEditor = editor.as_ref();

            self.error_ids = editor.delta_decorations(&self.error_ids, &error_ids);
            self.breakpoint_ids = editor.delta_decorations(&self.breakpoint_ids, &breakpoint_ids);
            self.debug_line_ids = editor.delta_decorations(&self.debug_line_ids, &debug_line_ids);
        });
    }
}

pub fn setup_breakpoints(editor: &CodeEditor) -> DisposableClosure<dyn FnMut(IEditorMouseEvent)> {
    editor.on_mouse_down(|e| {
        if let MouseTargetType::GutterLineNumbers = e.target().type_() {
            let Some(line_number) = e.target().position() else {
                return;
            };
            let line_number = line_number.line_number() as usize;

            dispatch().reduce_mut(|s: &mut Store| {
                if s.breakpoints.contains(&line_number) {
                    s.breakpoints.remove(&line_number);
                } else {
                    s.breakpoints.insert(line_number);
                }
            });
        }
    })
}

fn draw_error(error: &OutputComponentErrors) -> Array {
    let (kind, line, error_classname) = match &error {
        OutputComponentErrors::InterpretError(e) => (
            format!("{:?}", e.kind),
            e.line as f64,
            "runtime-error-line-highlight",
        ),
        OutputComponentErrors::ParseError(e) => (
            format!("{:?}", e.kind),
            e.line as f64,
            "syntax-error-line-highlight",
        ),
    };

    let new_decorations = Array::new();

    let options: IModelDecorationOptions = Object::new().unchecked_into();
    options.set_class_name(Some(error_classname));
    options.set_is_whole_line(Some(true));
    options.set_hover_message(&JsValue::from_str(kind.as_str()));
    options.set_glyph_margin_hover_message(&JsValue::from_str(kind.as_str()));
    options.set_glyph_margin_class_name(Some("error-glyph-margin"));

    let irange: IRange = Range::new(line, 1., line, 3.).unchecked_into();

    let decoration: IModelDeltaDecoration = Object::new().unchecked_into();
    decoration.set_options(&options);
    decoration.set_range(&irange);

    new_decorations.push(&decoration);

    new_decorations
}

fn breakpoints_dec(line: f64) -> Array {
    let new_decorations = Array::new();

    let options: IModelDecorationOptions = Object::new().unchecked_into();
    options.set_glyph_margin_class_name(Some("breakpoint"));
    options.set_is_whole_line(Some(true));

    let irange: IRange = Range::new(line, 1., line, 3.).unchecked_into();

    let decoration: IModelDeltaDecoration = Object::new().unchecked_into();
    decoration.set_options(&options);
    decoration.set_range(&irange);

    new_decorations.push(&decoration);

    new_decorations
}

fn debug_line_dec(line: f64) -> Array {
    let new_decorations = Array::new();

    let options: IModelDecorationOptions = Object::new().unchecked_into();
    options.set_is_whole_line(Some(true));
    options.set_class_name(Some("debug-line-highlight"));

    let irange: IRange = Range::new(line, 1., line, 3.).unchecked_into();

    let decoration: IModelDeltaDecoration = Object::new().unchecked_into();
    decoration.set_options(&options);
    decoration.set_range(&irange);

    new_decorations.push(&decoration);

    new_decorations
}
