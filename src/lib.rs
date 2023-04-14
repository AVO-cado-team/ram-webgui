mod app;
mod code_editor;
mod error_display;
mod footer;
mod io;
mod memory;
mod header;
mod show_content;

use app::App;
use wasm_bindgen::prelude::*;

pub fn run_app() -> Result<(), JsValue> {
  yew::Renderer::<App>::new().render();
  Ok(())
}
