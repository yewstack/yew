#![recursion_limit = "128"]
use stdweb::web::{document, IElement};
#[cfg(feature = "wasm_test")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::html::Scope;
use yew::virtual_dom::vtag::{VTag, HTML_NAMESPACE, SVG_NAMESPACE};
use yew::virtual_dom::{VDiff, VNode};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[cfg(feature = "wasm_test")]
wasm_bindgen_test_configure!(run_in_browser);

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

    fn view(&self) -> Html<Self> {
        unimplemented!();
    }
}

struct CompInt;

impl Component for CompInt {
    type Message = u32;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CompInt
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }

    fn view(&self) -> Html<Self> {
        unimplemented!();
    }
}

struct CompBool;

impl Component for CompBool {
    type Message = bool;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CompBool
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }

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
        <div a="test"></div>
    };

    let b: VNode<Comp> = html! {
        <div a="test"></div>
    };

    let c: VNode<Comp> = html! {
        <div a="fail"></div>
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
        <div class="test"></div>
    };

    let b: VNode<Comp> = html! {
        <div class="test"></div>
    };

    let c: VNode<Comp> = html! {
        <div class="fail"></div>
    };

    let d: VNode<Comp> = html! {
        <div class=format!("fail")></div>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_eq!(c, d);
}

#[test]
fn classes_from_local_variables() {
    let a: VNode<Comp> = html! {
        <div class=("class-1", "class-2")></div>
    };

    let class_2 = "class-2";
    let b: VNode<Comp> = html! {
        <div class=("class-1", class_2)></div>
    };

    let class_2_fmt = format!("class-{}", 2);
    let c: VNode<Comp> = html! {
        <div class=("class-1", class_2_fmt)></div>
    };

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn supports_multiple_classes_string() {
    let a: VNode<Comp> = html! {
        <div class="class-1 class-2   class-3"></div>
    };

    let b: VNode<Comp> = html! {
        <div class="class-2 class-3 class-1"></div>
    };

    assert_ne!(a, b);

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
fn supports_multiple_classes_vec() {
    let mut classes = vec!["class-1"];
    classes.push("class-2");
    let a: VNode<Comp> = html! {
        <div class=classes></div>
    };

    if let VNode::VTag(vtag) = a {
        println!("{:?}", vtag.classes);
        assert!(vtag.classes.contains("class-1"));
        assert!(vtag.classes.contains("class-2"));
        assert!(!vtag.classes.contains("class-3"));
    } else {
        panic!("vtag expected");
    }
}

fn assert_vtag(node: &mut VNode<Comp>) -> &mut VTag<Comp> {
    if let VNode::VTag(vtag) = node {
        return vtag;
    }
    panic!("should be vtag");
}

fn assert_namespace(vtag: &VTag<Comp>, namespace: &'static str) {
    assert_eq!(
        vtag.reference.as_ref().unwrap().namespace_uri().unwrap(),
        namespace
    );
}

#[test]
fn supports_svg() {
    let scope = Scope::new();
    let div_el = document().create_element("div").unwrap();
    let svg_el = document().create_element_ns(SVG_NAMESPACE, "svg").unwrap();

    let mut g_node: VNode<Comp> = html! { <g></g> };
    let path_node: VNode<Comp> = html! { <path></path> };
    let mut svg_node: VNode<Comp> = html! { <svg>{path_node}</svg> };

    let svg_tag = assert_vtag(&mut svg_node);
    svg_tag.apply(&div_el, None, None, &scope);
    assert_namespace(svg_tag, SVG_NAMESPACE);
    let path_tag = assert_vtag(svg_tag.children.get_mut(0).unwrap());
    assert_namespace(path_tag, SVG_NAMESPACE);

    let g_tag = assert_vtag(&mut g_node);
    g_tag.apply(&div_el, None, None, &scope);
    assert_namespace(g_tag, HTML_NAMESPACE);
    g_tag.reference = None;

    g_tag.apply(&svg_el, None, None, &scope);
    assert_namespace(g_tag, SVG_NAMESPACE);
}

#[test]
fn keeps_order_of_classes() {
    let a: VNode<Comp> = html! {
        <div class="class-1 class-2   class-3",></div>
    };

    if let VNode::VTag(vtag) = a {
        println!("{:?}", vtag.classes);
        assert_eq!(vtag.classes.to_string(), "class-1 class-2 class-3");
    }
}

#[test]
fn it_compares_values() {
    let a: VNode<Comp> = html! {
        <input value="test"/>
    };

    let b: VNode<Comp> = html! {
        <input value="test"/>
    };

    let c: VNode<Comp> = html! {
        <input value="fail"/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_kinds() {
    let a: VNode<Comp> = html! {
        <input type="text"/>
    };

    let b: VNode<Comp> = html! {
        <input type="text"/>
    };

    let c: VNode<Comp> = html! {
        <input type="hidden"/>
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_compares_checked() {
    let a: VNode<Comp> = html! {
        <input type="checkbox" checked=false />
    };

    let b: VNode<Comp> = html! {
        <input type="checkbox" checked=false />
    };

    let c: VNode<Comp> = html! {
        <input type="checkbox" checked=true />
    };

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn it_allows_aria_attributes() {
    let a: VNode<Comp> = html! {
        <p aria-controls="it-works">
            <a class="btn btn-primary"
               data-toggle="collapse"
               href="#collapseExample"
               role="button"
               aria-expanded="false"
               aria-controls="collapseExample">
                { "Link with href" }
            </a>
            <button class="btn btn-primary"
                    type="button"
                    data-toggle="collapse"
                    data-target="#collapseExample"
                    aria-expanded="false"
                    aria-controls="collapseExample">
                { "Button with data-target" }
            </button>
            <div own-attribute-with-multiple-parts="works" />
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

#[test]
fn it_checks_mixed_closing_tags() {
    let a: VNode<Comp> = html! { <div> <div/>      </div> };
    let b: VNode<Comp> = html! { <div> <div></div> </div> };
    assert_eq!(a, b);

    let a: VNode<Comp> = html! { <div> <div data-val={ 2 / 1 }/>  </div> };
    let b: VNode<Comp> = html! { <div> <div data-val={ 2 }></div> </div> };
    assert_eq!(a, b);

    let a: VNode<Comp> = html! { <div> <div data-val={ 2 > 1 }/>  </div> };
    let b: VNode<Comp> = html! { <div> <div data-val={ true }></div> </div> };
    assert_eq!(a, b);

    let a: VNode<CompInt> = html! { <div> <div onblur=|_| 2 / 1/>  </div> };
    let b: VNode<CompInt> = html! { <div> <div onblur=|_| 2></div> </div> };
    assert_eq!(a, b);
}

#[test]
fn it_checks_misleading_gt() {
    let a: VNode<CompBool> = html! { <div> <div onblur=|_|   2 > 1   /> </div> };
    let b: VNode<CompBool> = html! { <div> <div onblur=|_| { 2 > 1 } /> </div> };
    let c: VNode<CompBool> = html! { <div> <div onblur=|_| ( 2 > 1 ) /> </div> };
    let d: VNode<CompBool> = html! { <div> <div onblur=|_| true ></div> </div> };
    assert_eq!(a, b);
    assert_eq!(a, c);
    assert_eq!(a, d);

    let a: VNode<CompBool> = html! { <div><div onblur=|_|  2 > 1 />      </div> };
    let b: VNode<CompBool> = html! { <div><div onblur=|_| { true }></div></div> };
    assert_eq!(a, b);

    let a: VNode<CompInt> = html! { <div> <a onblur=|_| -> u32 { 0 } />  </div> };
    let b: VNode<CompInt> = html! { <div> <a onblur=|_| 0></a> </div> };
    assert_eq!(a, b);
}
