#![recursion_limit = "256"]

mod lib;
use lib::Model;

extern crate serde_derive;
extern crate common;

fn main() {
    yew::start_app::<Model>();
}
