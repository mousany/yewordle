mod app;
mod components;
mod composables;
mod types;
mod views;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
