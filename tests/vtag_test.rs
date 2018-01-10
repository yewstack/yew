#[macro_use]
extern crate yew;

use yew::virtual_dom::VTag;

#[test]
fn it_compares_tags() {
    let a: VTag<()> = html! {
        <div></div>
    };

    let b: VTag<()> = html! {
        <div></div>
    };

    let c: VTag<()> = html! {
        <p></p>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_text() {
    let a: VTag<()> = html! {
        <div>{ "correct" }</div>
    };

    let b: VTag<()> = html! {
        <div>{ "correct" }</div>
    };

    let c: VTag<()> = html! {
        <div>{ "incorrect" }</div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_attributes() {
    let a: VTag<()> = html! {
        <div a="test",></div>
    };

    let b: VTag<()> = html! {
        <div a="test",></div>
    };

    let c: VTag<()> = html! {
        <div a="fail",></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_children() {
    let a: VTag<()> = html! {
        <div>
            <p></p>
        </div>
    };

    let b: VTag<()> = html! {
        <div>
            <p></p>
        </div>
    };

    let c: VTag<()> = html! {
        <div>
            <span></span>
        </div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_classes() {
    let a: VTag<()> = html! {
        <div class="test",></div>
    };

    let b: VTag<()> = html! {
        <div class="test",></div>
    };

    let c: VTag<()> = html! {
        <div class="fail",></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_values() {
    let a: VTag<()> = html! {
        <input value="test",/>
    };

    let b: VTag<()> = html! {
        <input value="test",/>
    };

    let c: VTag<()> = html! {
        <input value="fail",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_kinds() {
    let a: VTag<()> = html! {
        <input type="text",/>
    };

    let b: VTag<()> = html! {
        <input type="text",/>
    };

    let c: VTag<()> = html! {
        <input type="hidden",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_checked() {
    let a: VTag<()> = html! {
        <input type="checkbox", checked=false,/>
    };

    let b: VTag<()> = html! {
        <input type="checkbox", checked=false,/>
    };

    let c: VTag<()> = html! {
        <input type="checkbox", checked=true,/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_allows_aria_attributes() {
    let a: VTag<()> = html! {
        <p aria-controls="it-works",>
            <a class="btn btn-primary",
               data-toggle="collapse",
               href="#collapseExample",
               role="button",
               aria-expanded="false",
               aria-controls="collapseExample",>
                { "Link with href" }
            </a>
            <button class="btn btn-primary",
                    type="button",
                    data-toggle="collapse",
                    data-target="#collapseExample",
                    aria-expanded="false",
                    aria-controls="collapseExample",>
                { "Button with data-target" }
            </button>
            <div own-attribute-with-multiple-parts="works", />
        </p>
    };
    assert!(a.attributes.contains_key("aria-controls"));
    assert_eq!(a.attributes.get("aria-controls"), Some(&"it-works".into()));
}
