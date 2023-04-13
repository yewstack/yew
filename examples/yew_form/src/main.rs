mod api;
mod app;
mod components;
mod pages;
mod router;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
