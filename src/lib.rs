mod app;
mod code_editor;

use app::App;
use wasm_bindgen::prelude::*;

pub fn run_app() -> Result<(), JsValue> {
  yew::Renderer::<App>::new().render();
  Ok(())
}
