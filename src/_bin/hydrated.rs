#[cfg(feature = "ssr")]
use ram_webgui::App;

#[cfg(feature = "ssr")]
fn main() {
    let dehydrated_app = yew::ServerRenderer::<App>::new().render();
    let hydrated = futures::executor::block_on(dehydrated_app);
    println!("{}", hydrated);
}

#[cfg(not(feature = "ssr"))]
fn main() {
    println!("SSR Is not enabled.");
}
