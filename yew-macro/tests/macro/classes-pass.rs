use yew::prelude::*;

fn compile_pass() {
    classes!("one", "two");
    classes!("one");
    classes!();

    classes!(vec!["one", "two"]);
    classes!(vec!["one"]);
    classes!(Vec::<&'static str>::new());
    classes!(vec!["one"], vec!["two"]);

    let some = Some("one");
    let none: Option<&'static str> = None;
    classes!(some, none);

    let string = "one".to_string();
    classes!(string.clone());
    classes!(string, "two", vec!["three"]);

    let array = ["one", "two"];
    classes!(&array[..]);
}

fn main() {}
