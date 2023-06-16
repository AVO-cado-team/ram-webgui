use js_sys::Object;
use monaco::sys::editor;
use monaco::sys::languages;
use monaco::sys::languages::ILanguageExtensionPoint;
use wasm_bindgen::{prelude::*, JsCast};

pub const LANG_ID: &str = "ram";
pub const THEME: &str = "ram-theme";
pub const THEME_JSON: &str = include_str!("../assets/theme.json");

pub fn register_ram() {
    languages::register(&language());
    languages::set_monarch_tokens_provider(LANG_ID, &make_tokens_provider().into());
    editor::define_theme(THEME, &load_theme(THEME_JSON).unchecked_into())
        .expect("Defining theme failed.");
    languages::register_completion_item_provider(
        LANG_ID,
        &completion_items_provider().unchecked_into(),
    );
}

fn language() -> ILanguageExtensionPoint {
    let lang: ILanguageExtensionPoint = Object::new().unchecked_into();
    lang.set_id(LANG_ID);
    lang
}

#[wasm_bindgen(module = "/js/completionItemProvider.js")]
extern "C" {
    #[wasm_bindgen(js_name = "completionItemsProvider")]
    fn completion_items_provider() -> Object;
}

#[wasm_bindgen(module = "/js/monarchTokensProvider.js")]
extern "C" {
    #[wasm_bindgen(js_name = "makeTokensProvider")]
    fn make_tokens_provider() -> Object;
}

#[wasm_bindgen(module = "/js/theme.js")]
extern "C" {
    #[wasm_bindgen(js_name = "loadTheme")]
    fn load_theme(json: &str) -> Object;
}
