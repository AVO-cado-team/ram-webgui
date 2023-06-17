// use ram_webgui::MyApp;
use ram_webgui::App;

fn main() {
    let dehydrated_app = yew::ServerRenderer::<App>::new().render();
    let hydrated = futures::executor::block_on(dehydrated_app);
    println!("{}", hydrated);
}
