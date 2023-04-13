mod components;
mod pages;
mod api;
mod app;
mod router;

fn main() {
    yew::Renderer::<app::App>::new().render();
}