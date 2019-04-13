extern crate yew;

use yew::virtual_dom::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }
}

impl Renderable<Comp> for Comp {
    fn view(&self) -> Html<Self> {
        unimplemented!();
    }
}

#[test]
fn it_compares_tags() {
    let a: VNode<Comp> = html! {
        <div></div>
    };

    let b: VNode<Comp> = html! {
        <div></div>
    };

    let c: VNode<Comp> = html! {
        <p></p>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_text() {
    let a: VNode<Comp> = html! {
        <div>{ "correct" }</div>
    };

    let b: VNode<Comp> = html! {
        <div>{ "correct" }</div>
    };

    let c: VNode<Comp> = html! {
        <div>{ "incorrect" }</div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_attributes() {
    let a: VNode<Comp> = html! {
        <div a="test",></div>
    };

    let b: VNode<Comp> = html! {
        <div a="test",></div>
    };

    let c: VNode<Comp> = html! {
        <div a="fail",></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_children() {
    let a: VNode<Comp> = html! {
        <div>
            <p></p>
        </div>
    };

    let b: VNode<Comp> = html! {
        <div>
            <p></p>
        </div>
    };

    let c: VNode<Comp> = html! {
        <div>
            <span></span>
        </div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_classes() {
    let a: VNode<Comp> = html! {
        <div class="test",></div>
    };

    let b: VNode<Comp> = html! {
        <div class="test",></div>
    };

    let c: VNode<Comp> = html! {
        <div class="fail",></div>
    };

    let d: VNode<Comp> = html! {
        <div class=format!("fail"),></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_eq!(c, d);
}

#[test]
fn classes_from_local_variables() {
    let a: VNode<Comp> = html! {
        <div class=("class-1", "class-2"),></div>
    };

    let class_2 = "class-2";
    let b: VNode<Comp> = html! {
        <div class=("class-1", class_2),></div>
    };

    let class_2_fmt = format!("class-{}", 2);
    let c: VNode<Comp> = html! {
        <div class=("class-1", class_2_fmt),></div>
    };

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn supports_multiple_classes_string() {
    let a: VNode<Comp> = html! {
        <div class="class-1 class-2   class-3",></div>
    };

    let b: VNode<Comp> = html! {
        <div class="class-2 class-3 class-1",></div>
    };

    assert_eq!(a, b);

    if let VNode::VTag(vtag) = a {
        println!("{:?}", vtag.classes);
        assert!(vtag.classes.contains("class-1"));
        assert!(vtag.classes.contains("class-2"));
        assert!(vtag.classes.contains("class-3"));
    } else {
        panic!("vtag expected");
    }
}

#[test]
fn it_compares_values() {
    let a: VNode<Comp> = html! {
        <input value="test",/>
    };

    let b: VNode<Comp> = html! {
        <input value="test",/>
    };

    let c: VNode<Comp> = html! {
        <input value="fail",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_kinds() {
    let a: VNode<Comp> = html! {
        <input type="text",/>
    };

    let b: VNode<Comp> = html! {
        <input type="text",/>
    };

    let c: VNode<Comp> = html! {
        <input type="hidden",/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_checked() {
    let a: VNode<Comp> = html! {
        <input type="checkbox", checked=false,/>
    };

    let b: VNode<Comp> = html! {
        <input type="checkbox", checked=false,/>
    };

    let c: VNode<Comp> = html! {
        <input type="checkbox", checked=true,/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_allows_aria_attributes() {
    let a: VNode<Comp> = html! {
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
    if let VNode::VTag(vtag) = a {
        assert!(vtag.attributes.contains_key("aria-controls"));
        assert_eq!(
            vtag.attributes.get("aria-controls"),
            Some(&"it-works".into())
        );
    } else {
        panic!("vtag expected");
    }
}
