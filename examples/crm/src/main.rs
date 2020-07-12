#![recursion_limit = "256"]

mod lib;
use lib::Model;

extern crate common;
extern crate serde_derive;

fn main() {
    yew::start_app::<Model>();
}
