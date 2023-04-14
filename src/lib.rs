mod app;
mod code_editor;
mod error_display;
mod footer;
mod io;
mod memory;
mod header;
mod show_content;
mod monaco_ram;

use app::App;
use monaco_ram::register_ram;
use wasm_bindgen::prelude::*;

pub fn run_app() -> Result<(), JsValue> {
  register_ram();
  yew::Renderer::<App>::new().render();
  Ok(())
}
