extern crate chat_room;
extern crate web_logger;
extern crate yew;

use chat_room::Model;

fn main() {
    web_logger::init();
    yew::Renderer::<Model>::new().render();
}
