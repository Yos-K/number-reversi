use yew::prelude::*;
use views::page::header::Header;
use app::menu::Menu;

mod controller;
mod game;
mod views;
mod tests;
mod app;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
            <Menu />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}