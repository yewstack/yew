use yew::prelude::*;

fn compile_pass() {
    classes!(42);
    classes!(42.0);

    classes!("one" "two");

    classes!(vec![42]);

    let some = Some(42);
    let none: Option<u32> = None;
    classes!(some);
    classes!(none);

    classes!("one", 42);

    classes!("one", "two three", "four");
}

fn main() {}
