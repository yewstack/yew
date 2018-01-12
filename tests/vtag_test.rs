#[macro_use]
extern crate yew;

use yew::virtual_dom::VNode;

#[test]
fn it_compares_tags() {
    let a: VNode<(),()> = html! {
        <div></div>
    };

    let b: VNode<(),()> = html! {
        <div></div>
    };

    let c: VNode<(),()> = html! {
        <p></p>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_text() {
    let a: VNode<(),()> = html! {
        <div>{ "correct" }</div>
    };

    let b: VNode<(),()> = html! {
        <div>{ "correct" }</div>
    };

    let c: VNode<(),()> = html! {
        <div>{ "incorrect" }</div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_attributes() {
    let a: VNode<(),()> = html! {
        <div a="test",></div>
    };

    let b: VNode<(),()> = html! {
        <div a="test",></div>
    };

    let c: VNode<(),()> = html! {
        <div a="fail",></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_children() {
    let a: VNode<(),()> = html! {
        <div>
            <p></p>
        </div>
    };

    let b: VNode<(),()> = html! {
        <div>
            <p></p>
        </div>
    };

    let c: VNode<(),()> = html! {
        <div>
            <span></span>
        </div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_classes() {
    let a: VNode<(),()> = html! {
        <div class="test",></div>
    };

    let b: VNode<(),()> = html! {
        <div class="test",></div>
    };

    let c: VNode<(),()> = html! {
        <div class="fail",></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_values() {
    let a: VNode<(),()> = html! {
        <input value="test",/>
    };

    let b: VNode<(),()> = html! {
        <input value="test",/>
    };

    let c: VNode<(),()> = html! {
        <input value="fail",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_kinds() {
    let a: VNode<(),()> = html! {
        <input type="text",/>
    };

    let b: VNode<(),()> = html! {
        <input type="text",/>
    };

    let c: VNode<(),()> = html! {
        <input type="hidden",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_checked() {
    let a: VNode<(),()> = html! {
        <input type="checkbox", checked=false,/>
    };

    let b: VNode<(),()> = html! {
        <input type="checkbox", checked=false,/>
    };

    let c: VNode<(),()> = html! {
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
