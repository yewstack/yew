use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

#[hook]
fn use_deref_as_u32() -> impl Deref<Target = u32> {
    Rc::new(0)
}

fn main() {}
