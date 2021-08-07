#![no_implicit_prelude]

fn main() {
    let dyn_tag = || ::std::string::ToString::to_string("test");
    let mut next_extra_tag = {
        let mut it = ::std::iter::IntoIterator::into_iter(::std::vec!["a", "b"]);
        move || ::std::option::Option::unwrap(::std::iter::Iterator::next(&mut it))
    };

    ::yew::html! {
        <@{ dyn_tag() }>
            <@{ next_extra_tag() } class="extra-a"/>
            <@{ next_extra_tag() } class="extra-b"/>
        </@>
    };

    ::yew::html! {
        <@{
            let tag = dyn_tag();
            if tag == "test" {
                "div"
            } else {
                "a"
            }
        }/>
    };
}
