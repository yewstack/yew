#![allow(missing_docs)]
//! This module contains the items required for a statically typed VDOM

use std::collections::HashMap;
use std::rc::Rc;

use crate::macros::generate_element;

use crate::virtual_dom::{AttrValue, Key, Listener, VNode};
use crate::{NodeRef};

generate_element! {
    base;
    props: {
        href: AttrValue,
        target: AttrValue,
     }
}

generate_element! {
    head;
    props: {  }
}

generate_element! {
    link;
    props: {
        crossorigin: AttrValue,
        href: AttrValue,
        hreflang: AttrValue,
        importance: AttrValue,
        integrity: AttrValue,
        media: AttrValue,
        referrerpolicy: AttrValue,
        rel: AttrValue,
        sizes: AttrValue,
     }
}

generate_element! {
    meta;
    props: {
        charset: AttrValue,
        content: AttrValue,
        http_equiv: AttrValue,
        name: AttrValue,
     }
}

generate_element! {
    style;
    props: {
        media: AttrValue,
        scoped: AttrValue,
        type: AttrValue,
     }
}

generate_element! {
    title;
    props: {  }
}

generate_element! {
    body;
    props: {
        background: AttrValue,
        bgcolor: AttrValue,
     }
}

generate_element! {
    address;
    props: {  }
}

generate_element! {
    article;
    props: {  }
}

generate_element! {
    aside;
    props: {  }
}

generate_element! {
    footer;
    props: {  }
}

generate_element! {
    header;
    props: {  }
}

generate_element! {
    main;
    props: {  }
}

generate_element! {
    nav;
    props: {  }
}

generate_element! {
    section;
    props: {  }
}

generate_element! {
    blockquote;
    props: {
        cite: AttrValue,
     }
}

generate_element! {
    dd;
    props: {  }
}

generate_element! {
    div;
    props: {  }
}

generate_element! {
    dl;
    props: {  }
}

generate_element! {
    dt;
    props: {  }
}

generate_element! {
    figcaption;
    props: {  }
}

generate_element! {
    figure;
    props: {  }
}

generate_element! {
    hr;
    props: {
        align: AttrValue,
        color: AttrValue,
     }
}

generate_element! {
    li;
    props: {
        value: AttrValue,
     }
}

generate_element! {
    ol;
    props: {
        reversed: AttrValue,
        start: AttrValue,
     }
}

generate_element! {
    p;
    props: {  }
}

generate_element! {
    pre;
    props: {  }
}

generate_element! {
    ul;
    props: {  }
}

generate_element! {
    a;
    props: {
        download: AttrValue,
        href: AttrValue,
        hreflang: AttrValue,
        media: AttrValue,
        ping: AttrValue,
        referrerpolicy: AttrValue,
        rel: AttrValue,
        shape: AttrValue,
        target: AttrValue,
     }
}

generate_element! {
    abbr;
    props: {  }
}

generate_element! {
    b;
    props: {  }
}

generate_element! {
    bdi;
    props: {  }
}

generate_element! {
    bdo;
    props: {  }
}

generate_element! {
    br;
    props: {  }
}

generate_element! {
    cite;
    props: {  }
}

generate_element! {
    code;
    props: {  }
}

generate_element! {
    data;
    props: {
        value: AttrValue,
     }
}

generate_element! {
    dfn;
    props: {  }
}

generate_element! {
    em;
    props: {  }
}

generate_element! {
    i;
    props: {  }
}

generate_element! {
    kbd;
    props: {  }
}

generate_element! {
    mark;
    props: {  }
}

generate_element! {
    q;
    props: {
        cite: AttrValue,
     }
}

generate_element! {
    rp;
    props: {  }
}

generate_element! {
    rt;
    props: {  }
}

generate_element! {
    ruby;
    props: {  }
}

generate_element! {
    s;
    props: {  }
}

generate_element! {
    samp;
    props: {  }
}

generate_element! {
    small;
    props: {  }
}

generate_element! {
    span;
    props: {  }
}

generate_element! {
    strong;
    props: {  }
}

generate_element! {
    sub;
    props: {  }
}

generate_element! {
    sup;
    props: {  }
}

generate_element! {
    time;
    props: {
        datetime: AttrValue,
     }
}

generate_element! {
    u;
    props: {  }
}

generate_element! {
    var;
    props: {  }
}

generate_element! {
    wbr;
    props: {  }
}

generate_element! {
    area;
    props: {
        alt: AttrValue,
        coords: AttrValue,
        download: AttrValue,
        href: AttrValue,
        hreflang: AttrValue,
        media: AttrValue,
        ping: AttrValue,
        referrerpolicy: AttrValue,
        rel: AttrValue,
        shape: AttrValue,
        target: AttrValue,
     }
}

generate_element! {
    audio;
    props: {
        autoplay: AttrValue,
        buffered: AttrValue,
        controls: AttrValue,
        crossorigin: AttrValue,
        loop: AttrValue,
        muted: AttrValue,
        preload: AttrValue,
        src: AttrValue,
     }
}

generate_element! {
    img;
    props: {
        align: AttrValue,
        alt: AttrValue,
        border: AttrValue,
        crossorigin: AttrValue,
        decoding: AttrValue,
        height: AttrValue,
        importance: AttrValue,
        intrinsicsize: AttrValue,
        ismap: AttrValue,
        loading: AttrValue,
        referrerpolicy: AttrValue,
        sizes: AttrValue,
        src: AttrValue,
        srcset: AttrValue,
        usemap: AttrValue,
        width: AttrValue,
     }
}

generate_element! {
    map;
    props: {
        name: AttrValue,
     }
}

generate_element! {
    track;
    props: {
        default: AttrValue,
        kind: AttrValue,
        label: AttrValue,
        src: AttrValue,
        srclang: AttrValue,
     }
}

generate_element! {
    video;
    props: {
        autoplay: AttrValue,
        buffered: AttrValue,
        controls: AttrValue,
        crossorigin: AttrValue,
        height: AttrValue,
        loop: AttrValue,
        muted: AttrValue,
        poster: AttrValue,
        preload: AttrValue,
        src: AttrValue,
        width: AttrValue,
     }
}

generate_element! {
    embed;
    props: {
        height: AttrValue,
        src: AttrValue,
        type: AttrValue,
        width: AttrValue,
     }
}

generate_element! {
    iframe;
    props: {
        align: AttrValue,
        allow: AttrValue,
        csp: AttrValue,
        height: AttrValue,
        importance: AttrValue,
        loading: AttrValue,
        name: AttrValue,
        referrerpolicy: AttrValue,
        sandbox: AttrValue,
        src: AttrValue,
        srcdoc: AttrValue,
        width: AttrValue,
     }
}

generate_element! {
    object;
    props: {
        border: AttrValue,
        data: AttrValue,
        form: AttrValue,
        height: AttrValue,
        name: AttrValue,
        type: AttrValue,
        usemap: AttrValue,
        width: AttrValue,
     }
}

generate_element! {
    param;
    props: {
        name: AttrValue,
        value: AttrValue,
     }
}

generate_element! {
    picture;
    props: {  }
}

generate_element! {
    portal;
    props: {  }
}

generate_element! {
    source;
    props: {
        media: AttrValue,
        sizes: AttrValue,
        src: AttrValue,
        srcset: AttrValue,
        type: AttrValue,
     }
}

generate_element! {
    svg;
    props: {  }
}

generate_element! {
    math;
    props: {  }
}

generate_element! {
    canvas;
    props: {
        height: AttrValue,
        width: AttrValue,
     }
}

generate_element! {
    noscript;
    props: {  }
}

generate_element! {
    script;
    props: {
        async: AttrValue,
        charset: AttrValue,
        crossorigin: AttrValue,
        defer: AttrValue,
        importance: AttrValue,
        integrity: AttrValue,
        language: AttrValue,
        referrerpolicy: AttrValue,
        src: AttrValue,
        type: AttrValue,
     }
}

generate_element! {
    del;
    props: {
        cite: AttrValue,
        datetime: AttrValue,
     }
}

generate_element! {
    ins;
    props: {
        cite: AttrValue,
        datetime: AttrValue,
     }
}

generate_element! {
    caption;
    props: {
        align: AttrValue,
     }
}

generate_element! {
    col;
    props: {
        align: AttrValue,
        bgcolor: AttrValue,
        span: AttrValue,
     }
}

generate_element! {
    colgroup;
    props: {
        align: AttrValue,
        bgcolor: AttrValue,
        span: AttrValue,
     }
}

generate_element! {
    table;
    props: {
        align: AttrValue,
        background: AttrValue,
        bgcolor: AttrValue,
        border: AttrValue,
        summary: AttrValue,
     }
}

generate_element! {
    tbody;
    props: {
        align: AttrValue,
        bgcolor: AttrValue,
     }
}

generate_element! {
    td;
    props: {
        align: AttrValue,
        background: AttrValue,
        bgcolor: AttrValue,
        colspan: AttrValue,
        headers: AttrValue,
        rowspan: AttrValue,
     }
}

generate_element! {
    tfoot;
    props: {
        align: AttrValue,
        bgcolor: AttrValue,
     }
}

generate_element! {
    th;
    props: {
        align: AttrValue,
        background: AttrValue,
        bgcolor: AttrValue,
        colspan: AttrValue,
        headers: AttrValue,
        rowspan: AttrValue,
        scope: AttrValue,
     }
}

generate_element! {
    thead;
    props: {
        align: AttrValue,
     }
}

generate_element! {
    tr;
    props: {
        align: AttrValue,
        bgcolor: AttrValue,
     }
}

generate_element! {
    button;
    props: {
        autofocus: AttrValue,
        disabled: AttrValue,
        form: AttrValue,
        formaction: AttrValue,
        formenctype: AttrValue,
        formmethod: AttrValue,
        formnovalidate: AttrValue,
        formtarget: AttrValue,
        name: AttrValue,
        type: AttrValue,
        value: AttrValue,
     }
}

generate_element! {
    datalist;
    props: {  }
}

generate_element! {
    fieldset;
    props: {
        disabled: AttrValue,
        form: AttrValue,
        name: AttrValue,
     }
}

generate_element! {
    form;
    props: {
        accept: AttrValue,
        accept_charset: AttrValue,
        action: AttrValue,
        autocomplete: AttrValue,
        enctype: AttrValue,
        method: AttrValue,
        name: AttrValue,
        novalidate: AttrValue,
        target: AttrValue,
     }
}

generate_element! {
    input;
    props: {
        accept: AttrValue,
        alt: AttrValue,
        autocomplete: AttrValue,
        autofocus: AttrValue,
        capture: AttrValue,
        checked: AttrValue,
        dirname: AttrValue,
        disabled: AttrValue,
        form: AttrValue,
        formaction: AttrValue,
        formenctype: AttrValue,
        formmethod: AttrValue,
        formnovalidate: AttrValue,
        formtarget: AttrValue,
        height: AttrValue,
        list: AttrValue,
        max: AttrValue,
        maxlength: AttrValue,
        minlength: AttrValue,
        min: AttrValue,
        multiple: AttrValue,
        name: AttrValue,
        pattern: AttrValue,
        placeholder: AttrValue,
        readonly: AttrValue,
        required: AttrValue,
        size: AttrValue,
        src: AttrValue,
        step: AttrValue,
        type: AttrValue,
        usemap: AttrValue,
        value: AttrValue,
        width: AttrValue,
     }
}

generate_element! {
    label;
    props: {
        for: AttrValue,
        form: AttrValue,
     }
}

generate_element! {
    legend;
    props: {  }
}

generate_element! {
    meter;
    props: {
        form: AttrValue,
        high: AttrValue,
        low: AttrValue,
        max: AttrValue,
        min: AttrValue,
        optimum: AttrValue,
        value: AttrValue,
     }
}

generate_element! {
    optgroup;
    props: {
        disabled: AttrValue,
        label: AttrValue,
     }
}

generate_element! {
    option;
    props: {
        disabled: AttrValue,
        label: AttrValue,
        selected: AttrValue,
        value: AttrValue,
     }
}

generate_element! {
    output;
    props: {
        for: AttrValue,
        form: AttrValue,
        name: AttrValue,
     }
}

generate_element! {
    progress;
    props: {
        form: AttrValue,
        max: AttrValue,
        value: AttrValue,
     }
}

generate_element! {
    select;
    props: {
        autocomplete: AttrValue,
        autofocus: AttrValue,
        disabled: AttrValue,
        form: AttrValue,
        multiple: AttrValue,
        name: AttrValue,
        required: AttrValue,
        size: AttrValue,
     }
}

generate_element! {
    textarea;
    props: {
        autocomplete: AttrValue,
        autofocus: AttrValue,
        cols: AttrValue,
        dirname: AttrValue,
        disabled: AttrValue,
        enterkeyhint: AttrValue,
        form: AttrValue,
        inputmode: AttrValue,
        maxlength: AttrValue,
        minlength: AttrValue,
        name: AttrValue,
        placeholder: AttrValue,
        readonly: AttrValue,
        required: AttrValue,
        rows: AttrValue,
        wrap: AttrValue,
     }
}

generate_element! {
    details;
    props: {
        open: AttrValue,
     }
}

generate_element! {
    dialog;
    props: {  }
}

generate_element! {
    menu;
    props: {
        type: AttrValue,
     }
}

generate_element! {
    summary;
    props: {  }
}

generate_element! {
    slot;
    props: {  }
}

generate_element! {
    template;
    props: {  }
}

generate_element! {
    acronym;
    props: {  }
}

generate_element! {
    applet;
    props: {
        align: AttrValue,
        alt: AttrValue,
        code: AttrValue,
        codebase: AttrValue,
     }
}

generate_element! {
    basefont;
    props: {
        color: AttrValue,
     }
}

generate_element! {
    bgsound;
    props: {
        loop: AttrValue,
     }
}

generate_element! {
    big;
    props: {  }
}

generate_element! {
    blink;
    props: {  }
}

generate_element! {
    center;
    props: {  }
}

generate_element! {
    content;
    props: {  }
}

generate_element! {
    dir;
    props: {  }
}

generate_element! {
    font;
    props: {
        color: AttrValue,
     }
}

generate_element! {
    frame;
    props: {  }
}

generate_element! {
    frameset;
    props: {  }
}

generate_element! {
    hgroup;
    props: {  }
}

generate_element! {
    image;
    props: {  }
}

generate_element! {
    keygen;
    props: {
        autofocus: AttrValue,
        challenge: AttrValue,
        disabled: AttrValue,
        form: AttrValue,
        keytype: AttrValue,
        name: AttrValue,
     }
}

generate_element! {
    marquee;
    props: {
        bgcolor: AttrValue,
        loop: AttrValue,
     }
}

generate_element! {
    menuitem;
    props: {  }
}

generate_element! {
    nobr;
    props: {  }
}

generate_element! {
    noembed;
    props: {  }
}

generate_element! {
    noframes;
    props: {  }
}

generate_element! {
    plaintext;
    props: {  }
}

generate_element! {
    rb;
    props: {  }
}

generate_element! {
    rtc;
    props: {  }
}

generate_element! {
    shadow;
    props: {  }
}

generate_element! {
    spacer;
    props: {  }
}

generate_element! {
    strike;
    props: {  }
}

generate_element! {
    tt;
    props: {  }
}

generate_element! {
    xmp;
    props: {  }
}

generate_element! {
    h1;
    props: {  }
}

generate_element! {
    h2;
    props: {  }
}

generate_element! {
    h3;
    props: {  }
}

generate_element! {
    h4;
    props: {  }
}

generate_element! {
    h5;
    props: {  }
}

generate_element! {
    h6;
    props: {  }
}


/// Metadata of an HTML element
///
/// A [Component](crate::html::Component) is generated using this data for every element.
#[derive(Debug)]
pub struct ElementData {
    node_ref: NodeRef,
    attributes: HashMap<&'static str, AttrValue>,
    listeners: Vec<Option<Rc<dyn Listener>>>,
    key: Option<Key>,
    children: Vec<VNode>,
}

#[cfg(all(test, feature = "wasm_test"))]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::{function_component, html, props, Callback, Html, Properties};

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn it_works() {
        use super::button as Btn;

        const TEXT: &'static str = "Inner Text";
        const CLICKED_TEXT: &'static str = "Clicked Text";

        #[derive(PartialEq, Properties)]
        struct Props {
            on_click: Callback<web_sys::MouseEvent>,
        }

        #[function_component]
        fn Comp(props: &Props) -> Html {
            html! {
                <Btn class="ccc" name="yes" on_click={props.on_click.clone()}>{ TEXT }</Btn>
            }
        }

        let event_data = Rc::new(RefCell::new(None));

        let on_click = {
            let event_data = Rc::clone(&event_data);
            Callback::from(move |_| {
                (*event_data).borrow_mut().replace(CLICKED_TEXT);
            })
        };

        let document = gloo_utils::document();
        yew::start_app_with_props_in_element::<Comp>(
            document.get_element_by_id("output").unwrap(),
            props!(Props { on_click: on_click }),
        );
        let button = gloo_utils::document()
            .query_selector("#output button")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>();
        assert_eq!(button.get_attribute("class").unwrap(), "ccc");
        assert_eq!(button.get_attribute("name").unwrap(), "yes");
        assert_eq!(button.inner_text(), TEXT);

        button.click();
        let data = *event_data.borrow();
        assert_eq!(data, Some(CLICKED_TEXT))
    }
}
