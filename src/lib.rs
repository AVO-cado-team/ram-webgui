#![allow(non_camel_case_types)]

mod about_popup;
mod app;
mod code_editor;
mod code_runner;
mod header;
mod io;
mod memory;
mod monaco_ram;
mod utils;

use std::panic;

pub use app::App;
use wasm_bindgen::prelude::*;

pub fn run_app() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(not(feature = "ssr"))]
    yew::Renderer::<App>::new().render();

    #[cfg(feature = "ssr")]
    yew::Renderer::<App>::new().hydrate();

    Ok(())
}
