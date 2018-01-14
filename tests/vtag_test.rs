#[macro_use]
extern crate yew;

use yew::html::{Html, Component, Env, ShouldRender};
use yew::virtual_dom::VNode;

type Ctx = ();

struct Comp;

impl Component<Ctx> for Comp {
    type Msg = ();
    type Properties = ();

    fn create(_: &mut Env<Ctx, Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Msg, _: &mut Env<Ctx, Self>) -> ShouldRender {
        unimplemented!();
    }

    fn view(&self) -> Html<Ctx, Self> {
        unimplemented!();
    }
}

#[test]
fn it_compares_tags() {
    let a: VNode<Ctx, Comp> = html! {
        <div></div>
    };

    let b: VNode<Ctx, Comp> = html! {
        <div></div>
    };

    let c: VNode<Ctx, Comp> = html! {
        <p></p>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_text() {
    let a: VNode<Ctx, Comp> = html! {
        <div>{ "correct" }</div>
    };

    let b: VNode<Ctx, Comp> = html! {
        <div>{ "correct" }</div>
    };

    let c: VNode<Ctx, Comp> = html! {
        <div>{ "incorrect" }</div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_attributes() {
    let a: VNode<Ctx, Comp> = html! {
        <div a="test",></div>
    };

    let b: VNode<Ctx, Comp> = html! {
        <div a="test",></div>
    };

    let c: VNode<Ctx, Comp> = html! {
        <div a="fail",></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_children() {
    let a: VNode<Ctx, Comp> = html! {
        <div>
            <p></p>
        </div>
    };

    let b: VNode<Ctx, Comp> = html! {
        <div>
            <p></p>
        </div>
    };

    let c: VNode<Ctx, Comp> = html! {
        <div>
            <span></span>
        </div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_classes() {
    let a: VNode<Ctx, Comp> = html! {
        <div class="test",></div>
    };

    let b: VNode<Ctx, Comp> = html! {
        <div class="test",></div>
    };

    let c: VNode<Ctx, Comp> = html! {
        <div class="fail",></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_values() {
    let a: VNode<Ctx, Comp> = html! {
        <input value="test",/>
    };

    let b: VNode<Ctx, Comp> = html! {
        <input value="test",/>
    };

    let c: VNode<Ctx, Comp> = html! {
        <input value="fail",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_kinds() {
    let a: VNode<Ctx, Comp> = html! {
        <input type="text",/>
    };

    let b: VNode<Ctx, Comp> = html! {
        <input type="text",/>
    };

    let c: VNode<Ctx, Comp> = html! {
        <input type="hidden",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_checked() {
    let a: VNode<Ctx, Comp> = html! {
        <input type="checkbox", checked=false,/>
    };

    let b: VNode<Ctx, Comp> = html! {
        <input type="checkbox", checked=false,/>
    };

    let c: VNode<Ctx, Comp> = html! {
        <input type="checkbox", checked=true,/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_allows_aria_attributes() {
    let a: VNode<Ctx, Comp> = html! {
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
    if let VNode::VTag { vtag, .. } = a {
        assert!(vtag.attributes.contains_key("aria-controls"));
        assert_eq!(vtag.attributes.get("aria-controls"), Some(&"it-works".into()));
    } else {
        panic!("vtag expected");
    }
}
