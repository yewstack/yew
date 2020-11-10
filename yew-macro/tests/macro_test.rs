use yew::html;

#[allow(dead_code)]
#[rustversion::attr(stable(1.45), test)]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/macro/html-block-pass.rs");
    t.compile_fail("tests/macro/html-block-fail.rs");

    t.pass("tests/macro/html-component-pass.rs");
    t.compile_fail("tests/macro/html-component-fail.rs");
    t.compile_fail("tests/macro/html-component-fail-unimplemented.rs");

    t.pass("tests/macro/html-iterable-pass.rs");
    t.compile_fail("tests/macro/html-iterable-fail.rs");

    t.pass("tests/macro/html-list-pass.rs");
    t.compile_fail("tests/macro/html-list-fail.rs");

    t.pass("tests/macro/html-node-pass.rs");
    t.compile_fail("tests/macro/html-node-fail.rs");

    t.pass("tests/macro/html-element-pass.rs");
    t.compile_fail("tests/macro/html-element-fail.rs");

    t.pass("tests/classes_macro/*-pass.rs");
    t.compile_fail("tests/classes_macro/*-fail.rs");
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
