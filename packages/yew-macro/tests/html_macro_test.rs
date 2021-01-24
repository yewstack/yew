use yew::html;

#[allow(dead_code)]
#[rustversion::attr(stable(1.45), test)]
fn html_macro() {
    let t = trybuild::TestCases::new();

    t.pass("tests/html_macro/*-pass.rs");
    t.compile_fail("tests/html_macro/*-fail.rs");
}

#[test]
#[should_panic(
    expected = "a dynamic tag tried to create a `<br>` tag with children. `<br>` is a void element which can't have any children."
)]
fn dynamic_tags_catch_void_elements() {
    html! {
        <@{"br"}>
            <span>{ "No children allowed" }</span>
        </@>
    };
}

#[test]
#[should_panic(expected = "a dynamic tag returned a tag name containing non ASCII characters: `❤`")]
fn dynamic_tags_catch_non_ascii() {
    html! {
        <@{"❤"}/>
    };
}
