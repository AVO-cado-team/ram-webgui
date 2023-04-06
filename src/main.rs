use ram_webgui::run_app;

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  run_app().expect("Failed to run app");
}
