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
mod store;

use std::panic::{self, PanicInfo};

pub use app::App;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    panic::set_hook(Box::new(handle_crash));

    wasm_logger::init(wasm_logger::Config::default());

    #[cfg(not(feature = "ssr"))]
    yew::Renderer::<App>::new().render();

    #[cfg(feature = "ssr")]
    yew::Renderer::<App>::new().hydrate();
}

fn handle_crash(_: &PanicInfo<'_>) {
    report_crash();
}

fn report_crash() {
    gloo::utils::body().set_inner_html(
        r#"
<style>
    body {
        font-family: Arial, sans-serif;
        background-color: #333; /* Dark background color */
        margin: 0;
        padding: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100vh;
        color: #fff; /* Light text color */
    }

    .error-container {
        text-align: center;
        padding: 20px;
        background-color: #555; /* Darker background color */
        border-radius: 8px;
        box-shadow: 0 0 10px rgba(255, 255, 255, 0.1); /* Light shadow color */
    }

    .error-header {
        margin-bottom: 1em;
    }

    .error-message {
        font-size: 18px;
        color: #ff0000;
        font-weight: bold;
    }
</style>
<div class="error-container">
    <h1 class="error-header">Oops! Something went wrong.</h1>
    <p class="error-message">We're sorry, something went wrong.</p>
</div>
"#,
    );
}
