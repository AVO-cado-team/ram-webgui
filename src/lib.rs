#![allow(non_camel_case_types)]

mod about_popup;
mod app;
mod code_editor;
mod header;
mod io;
mod memory;
mod monaco_ram;
mod utils;
mod code_runner;

use app::App;
use monaco_ram::register_ram;
use wasm_bindgen::prelude::*;

pub fn run_app() -> Result<(), JsValue> {
  register_ram();
  yew::Renderer::<App>::new().render();
  Ok(())
}
