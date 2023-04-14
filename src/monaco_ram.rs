use js_sys::Object;
use monaco::sys::languages::ILanguageExtensionPoint;
use wasm_bindgen::{prelude::*, JsCast};

pub const ID: &str = "ram";
pub const THEME: &str = "ram-theme";

pub fn register_ram() {
  monaco::sys::languages::register(&language());
  monaco::sys::languages::set_monarch_tokens_provider(ID, &make_tokens_provider().into());
  monaco::sys::editor::define_theme(THEME, &load_theme().unchecked_into())
    .expect("Defining theme failed.");
}

fn language() -> ILanguageExtensionPoint {
  let lang: ILanguageExtensionPoint = Object::new().unchecked_into();
  lang.set_id(ID);
  lang
}

#[wasm_bindgen(module = "/js/monarchTokensProvider.js")]
extern "C" {
  #[wasm_bindgen(js_name = "makeTokensProvider")]
  fn make_tokens_provider() -> Object;
}
#[wasm_bindgen(module = "/js/theme.js")]
extern "C" {
  #[wasm_bindgen(js_name = "loadTheme")]
  fn load_theme() -> Object;
}
