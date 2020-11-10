#![no_implicit_prelude]

fn compile_pass() {
    // multiple literals
    ::yew::classes!("one", "two");
    // single literal
    ::yew::classes!("one");
    // empty
    ::yew::classes!();

    // multiple expressions
    ::yew::classes!(::std::vec!["one"], ::std::vec!["two"]);
    // single expression
    ::yew::classes!(::std::vec!["one", "two"]);

    // optional classes
    ::yew::classes!(
        ::std::option::Option::Some("one"),
        ::std::option::Option::None::<&'static str>,
    );

    // mixed types
    {
        use ::std::borrow::ToOwned;
        ::yew::classes!("one".to_owned(), "two", ::std::vec!["three"]);
    }
}

fn main() {}
