#![allow(non_camel_case_types)]
#![forbid(unsafe_code)]

mod about_popup;
mod app;
mod code_runner;
mod header;
mod io {
    pub mod custom_reader;
    pub mod custom_writer;
    pub mod input;
    pub mod output;
}
#[cfg(not(feature = "ssr"))]
mod code_editor;
mod memory;
mod monaco_ram;
#[cfg(not(feature = "ssr"))]
mod monaco_tweaks;
#[cfg(feature = "ssr")]
mod code_editor {
    use yew::prelude::*;

    pub const DEFAULT_CODE: &str = "";

    #[derive(Properties, PartialEq)]
    pub struct Props {
        pub run_code: Callback<()>,
        pub read_only: bool,
        pub line: usize,
    }
    #[function_component(CustomEditor)]
    pub fn code_editor(_: &Props) -> Html {
        panic!("This component should not be used in server side rendering")
    }
}
mod store;
mod utils;

pub use app::App;
