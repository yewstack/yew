crate::doc_page!("Events", "/zh-Hant/docs/concepts/html/events",
    Content::new(vec![
        h2![text("Introduction")],
        p![
            text("Yew integrates with the "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/", code("web-sys")),
            text(" crate and uses the events from that crate. The "),
            link!("#event-types", text("table below")),
            text(" lists all of the "),
            code("web-sys"),
            text(" events that are accepted in the "),
            code("html!"),
            text(" macro."),
        ],
        p![
            text("You can still add a "),
            link!("/zh-Hant/docs/concepts/function-components/callbacks", code("Callback")),
            text(" for an event that is not listed in the table below, see "),
            link!("#manual-event-listener", text("Manual event listener")),
            text("."),
        ],
        h2![text("Event Types")],
        admonition!(AdmonitionType::Tip, None,
            p![
                text("All the event types mentioned in the following table are re-exported under "),
                code("yew::events"),
                text(". Using the types from "),
                code("yew::events"),
                text(" makes it easier to ensure version compatibility than \
                  if you were to manually include "),
                code("web-sys"),
                text(" as a dependency in your crate because you won't \
                  end up using a version which conflicts with the version that Yew specifies."),
            ],
        ),
        p![
            text("The event listener name is the expected name when adding an event "),
            code("Callback"),
            text(" in the "),
            code("html"),
            text(" macro:"),
        ],
        code_block("rust", r#"use yew::prelude::*;

html! {
<button onclick={Callback::from(|_| ())}>
//      ^^^^^^^ event listener name
{ "Click me!" }
</button>
};"#),
        p![
            text("The event name is the listener without the \"on\" prefix, therefore, the "),
            code("onclick"),
            text(" event listener listens for "),
            code("click"),
            text(" events. See the end of this page for a "),
            link!("#event-types", text("full list of available events")),
            text(" with their types."),
        ],
        h2_id!("event-bubbling", text("Event bubbling")),
        p![
            text("Events dispatched by Yew follow the virtual DOM hierarchy when bubbling up to listeners. Currently, only the bubbling phase \
              is supported for listeners. Note that the virtual DOM hierarchy is most often, but not always, identical to the actual \
              DOM hierarchy. The distinction is important when working with "),
            link!("/zh-Hant/docs/advanced-topics/portals", text("portals")),
            text(" and other more advanced techniques. The intuition for well implemented components should be that events bubble from children \
              to parents, so that the hierarchy in your coded "),
            code("html!"),
            text(" is the one observed by event handlers."),
        ],
        p![
            text("If you are not interested in event bubbling, you can turn it off by calling"),
        ],
        code_block("rust", r#"yew::set_event_bubbling(false);"#),
        p![
            italic![text("before")],
            text(" starting your app. This speeds up event handling, but some components may break from not receiving events they expect. \
              Use this with care!"),
        ],
        h2![text("Event delegation")],
        p![
            text("It can be surprising that event listeners are "),
            italic![text("not")],
            text(" directly registered on the element where they are rendered. Instead, events \
              are delegated from the subtree root of the Yew app. Still, events are delivered in their native form, and no synthetic \
              form is created. This can lead to mismatches between the event you'd expect in html listeners and those showing up in Yew."),
        ],
        ul![
            li![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.current_target", code("Event::current_target")),
                text(" points to the Yew subtree root instead of the element the listener is added on. Use "),
                link!("/zh-Hant/docs/concepts/function-components/node-refs", code("NodeRef")),
                text(" if you want access to the underlying "),
                code("HtmlElement"),
                text("."),
            ],
            li![
                link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.event_phase", code("Event::event_phase")),
                text(" is always "),
                code("Event::CAPTURING_PHASE"),
                text(". Internally, the event will behave as if it was in the bubbling \
                  phase, the event propagation is replayed and the event "),
                link!("#event-bubbling", text("bubbles up")),
                text(", i.e. event listeners higher up in \
                  the virtual DOM will trigger after event listeners below them. Currently, capturing listeners are not supported by Yew. \
                  This also means that events registered by Yew will usually fire before other event listeners."),
            ],
        ],
        h2![text("Typed event target")],
        admonition!(AdmonitionType::Caution, None,
            p![
                text("In this section "),
                bold![text("target")],
                text(" ("),
                link!("https://developer.mozilla.org/en-US/docs/Web/API/Event/target", code("Event.target")),
                text(") is always referring to the element at which the event was dispatched from."),
            ],
            p![
                text("This will "),
                bold![text("not")],
                text(" always be the element at which the "),
                code("Callback"),
                text(" is placed."),
            ],
        ),
        p![
            text("In event "),
            code("Callback"),
            text("s you may want to get the target of that event. For example, the "),
            code("change"),
            text(" event gives no information but is used to notify that something has changed."),
        ],
        p![
            text("In Yew getting the target element in the correct type can be done in a few ways and we will go through \
              them here. Calling "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target", code("web_sys::Event::target")),
            text(" on an event returns an optional "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html", code("web_sys::EventTarget")),
            text(" type, which might not seem very useful when you want to know the value of your input element."),
        ],
        p![
            text("In all the approaches below we are going to tackle the same problem, so it's clear where the approach \
              differs opposed to the problem at hand."),
        ],
        p![bold![text("The Problem:")]],
        p![
            text("We have an "),
            code("onchange"),
            text(" "),
            code("Callback"),
            text(" on my "),
            code("<input>"),
            text(" element and each time it is invoked we want to send \
              an update "),
            code("Msg"),
            text(" to our component."),
        ],
        p![
            text("Our "),
            code("Msg"),
            text(" enum looks like this:"),
        ],
        code_block("rust", r#"pub enum Msg {
InputValue(String),
}"#),
        h3![text("Using "), code("JsCast")],
        p![
            text("The "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html", code("wasm-bindgen")),
            text(" crate has a useful trait; "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", code("JsCast")),
            text(", which allows us to hop and skip our way to the type we want, as long as it implements "),
            code("JsCast"),
            text(". We can do this cautiously, which involves some runtime checks and failure types like "),
            code("Option"),
            text(" and "),
            code("Result"),
            text(", or we can do it dangerously."),
        ],
        code_block_title("toml", "Cargo.toml", r#"[dependencies]
# need wasm-bindgen for JsCast
wasm-bindgen = "0.2""#),
        code_block("rust", r#"//highlight-next-line
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
let input_value_handle = use_state(String::default);
let input_value = (*input_value_handle).clone();

let on_cautious_change = {
let input_value_handle = input_value_handle.clone();

Callback::from(move |e: Event| {
    // When events are created the target is undefined, it's only
    // when dispatched does the target get added.
    let target: Option<EventTarget> = e.target();
    // Events can bubble so this listener might catch events from child
    // elements which are not of type HtmlInputElement
    //highlight-next-line
    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

    if let Some(input) = input {
        input_value_handle.set(input.value());
    }
})
};

let on_dangerous_change = Callback::from(move |e: Event| {
let target: EventTarget = e
    .target()
    .expect("Event should have a target when dispatched");
// You must KNOW target is a HtmlInputElement, otherwise
// the call to value would be Undefined Behaviour (UB).
//highlight-next-line
input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
});

html! {
<>
    <label for="cautious-input">
        { "My cautious input:" }
        <input onchange={on_cautious_change}
            id="cautious-input"
            type="text"
            value={input_value.clone()}
        />
    </label>
    <label for="dangerous-input">
        { "My dangerous input:" }
        <input onchange={on_dangerous_change}
            id="dangerous-input"
            type="text"
            value={input_value}
        />
    </label>
</>
}
}"#),
        p![
            text("The methods from "),
            code("JsCast"),
            text(" are "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into", code("dyn_into")),
            text(" and "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into", code("unchecked_into")),
            text(" and they allowed us to go from "),
            code("EventTarget"),
            text(" to "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html", code("HtmlInputElement")),
            text(". The "),
            code("dyn_into"),
            text(" method is cautious because at runtime it will check whether the type is actually a "),
            code("HtmlInputElement"),
            text(" and if not return an "),
            code("Err(JsValue)"),
            text(", the "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html", code("JsValue")),
            text(" is a catch-all type and is essentially giving you back the object to try again."),
        ],
        p![
            text("At this point you might be thinking... when is the dangerous version ok to use? In the case above it \
              is safe"),
            sup![text("1")],
            text(" as we've set the "),
            code("Callback"),
            text(" on to an element with no children so the target can only be that same element."),
        ],
        p![
            italic![
                sup![text("1")],
                text(" As safe as anything can be when JS land is involved."),
            ],
        ],
        h3![text("Using "), code("TargetCast")],
        p![
            bold![text("It is highly recommended to read ")],
            link!("#using-jscast", text("Using JsCast")),
            bold![text(" first!")],
        ],
        admonition!(AdmonitionType::Note, None,
            p![
                code("TargetCast"),
                text(" was designed to feel very similar to "),
                code("JsCast"),
                text(" - this is to allow new users to get a feel \
                  for the behaviour of "),
                code("JsCast"),
                text(" but with the smaller scope of events and their targets."),
            ],
            p![
                code("TargetCast"),
                text(" vs "),
                code("JsCast"),
                text(" is purely preference, you will find that "),
                code("TargetCast"),
                text(" implements something similar to what you would using "),
                code("JsCast"),
                text("."),
            ],
        ),
        p![
            text("The "),
            code("TargetCast"),
            text(" trait is built on top of "),
            code("JsCast"),
            text(" and is specialized towards getting typed event targets from events."),
        ],
        p![
            code("TargetCast"),
            text(" comes with Yew so no need to add a dependency in order to use the trait methods on events \
              but it works in a very similar way to "),
            code("JsCast"),
            text("."),
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
let input_value_handle = use_state(String::default);
let input_value = (*input_value_handle).clone();

let on_cautious_change = {
let input_value_handle = input_value_handle.clone();

Callback::from(move |e: Event| {
    let input = e.target_dyn_into::<HtmlInputElement>();

    if let Some(input) = input {
        input_value_handle.set(input.value());
    }
})
};

let on_dangerous_change = Callback::from(move |e: Event| {
// You must KNOW target is a HtmlInputElement, otherwise
// the call to value would be Undefined Behaviour (UB).
//highlight-next-line
input_value_handle.set(e.target_unchecked_into::<HtmlInputElement>().value());
});

html! {
<>
    <label for="cautious-input">
        { "My cautious input:" }
        <input onchange={on_cautious_change}
            id="cautious-input"
            type="text"
            value={input_value.clone()}
        />
    </label>
    <label for="dangerous-input">
        { "My dangerous input:" }
        <input onchange={on_dangerous_change}
            id="dangerous-input"
            type="text"
            value={input_value}
        />
    </label>
</>
}
}"#),
        p![
            text("If you followed the advice above and read about "),
            code("JsCast"),
            text(", you can probably see that "),
            code("TargetCast::target_dyn_into"),
            text(" feels similar to "),
            code("JsCast::dyn_into"),
            text(" but specifically does the cast on the target of the event. "),
            code("TargetCast::target_unchecked_into"),
            text(" is similar to "),
            code("JsCast::unchecked_into"),
            text(", and as such all the same warnings above "),
            code("JsCast"),
            text(" apply to "),
            code("TargetCast"),
            text("."),
        ],
        h3![text("Using "), code("NodeRef")],
        p![
            link!("/zh-Hant/docs/concepts/function-components/node-refs", code("NodeRef")),
            text(" can be used instead of querying the event given to a "),
            code("Callback"),
            text("."),
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
//highlight-next-line
let input_node_ref = use_node_ref();

let input_value_handle = use_state(String::default);
let input_value = (*input_value_handle).clone();

let onchange = {
let input_node_ref = input_node_ref.clone();

Callback::from(move |_| {
    //highlight-next-line
    let input = input_node_ref.cast::<HtmlInputElement>();

    if let Some(input) = input {
        input_value_handle.set(input.value());
    }
})
};

html! {
<>
    <label for="my-input">
        { "My input:" }
        //highlight-next-line
        <input ref={input_node_ref}
            {onchange}
            id="my-input"
            type="text"
            value={input_value}
        />
    </label>
</>
}
}"#),
        p![
            text("Using "),
            code("NodeRef"),
            text(", you can ignore the event and use the "),
            code("NodeRef::cast"),
            text(" method to get an "),
            code("Option<HtmlInputElement>"),
            text(" - this is optional as calling "),
            code("cast"),
            text(" before the "),
            code("NodeRef"),
            text(" has been set, or when the type doesn't match will return "),
            code("None"),
            text("."),
        ],
        p![
            text("You might also see by using "),
            code("NodeRef"),
            text(" we don't have to send the "),
            code("String"),
            text(" back into state as we always have access to "),
            code("input_node_ref"),
            text(" - so we could do the following:"),
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
let input_node_ref = use_node_ref();

//highlight-start
let onchange = {
let input_node_ref = input_node_ref.clone();

Callback::from(move |_| {
    if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
        let value = input.value();
        // do something with value
    }
})
};
//highlight-end

html! {
<>
    <label for="my-input">
        { "My input:" }
        <input ref={input_node_ref}
            {onchange}
            id="my-input"
            type="text"
        />
    </label>
</>
}
}"#),
        p![
            text("Which approach you take depends on your component and your preferences, there is no blessed way per se."),
        ],
        h2_id!("manual-event-listener", text("Manual event listener")),
        p![
            text("You may want to listen to an event that is not supported by Yew's "),
            code("html"),
            text(" macro, see the "),
            link!("#event-types", text("supported events listed here")),
            text("."),
        ],
        p![
            text("In order to add an event listener to one of elements manually we need the help of "),
            link!("/zh-Hant/docs/concepts/function-components/node-refs", code("NodeRef")),
            text(" so that in "),
            code("use_effect_with_deps"),
            text(" we can add a listener using the "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/index.html", code("web-sys")),
            text(" and "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html", text("wasm-bindgen")),
            text(" API."),
        ],
        p![
            text("The examples below are going to show adding listeners for the made-up "),
            code("custard"),
            text(" event. All events either unsupported by yew or custom can be represented as a "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html", code("web_sys::Event")),
            text(". If you need to access a specific method or field on a custom / unsupported event then you can use the \
              methods of "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", code("JsCast")),
            text(" in order to convert to the type required."),
        ],
        h3![text("Using "), code("Closure"), text(" (verbose)")],
        p![
            text("Using the "),
            code("web-sys"),
            text(" and "),
            code("wasm-bindgen"),
            text(" API's directly for this can be a bit painful... so brace \
              yourself ("),
            link!("#using-gloo-concise", text("there is a more concise way thanks to gloo")),
            text(")."),
        ],
        code_block("rust", r#"use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
let div_node_ref = use_node_ref();

use_effect_with_deps(
{
    let div_node_ref = div_node_ref.clone();

    move |_| {
        let mut custard_listener = None;

        if let Some(element) = div_node_ref.cast::<HtmlElement>() {
            // Create your Callback as you normally would
            let oncustard = Callback::from(move |_: Event| {
                // do something about custard..
            });

            // Create a Closure from a Box<dyn Fn> - this has to be 'static
            let listener =
                Closure::<dyn Fn(Event)>::wrap(
                    Box::new(move |e: Event| oncustard.emit(e))
                );

            element
                .add_event_listener_with_callback(
                    "custard",
                    listener.as_ref().unchecked_ref()
                )
                .unwrap();

            custard_listener = Some(listener);
        }

        move || drop(custard_listener)
    }
},
div_node_ref.clone()
);

html! {
<div ref={div_node_ref} id="my-div"></div>
}
}"#),
        p![
            text("For more information on "),
            code("Closures"),
            text(", see "),
            link!("https://wasm-bindgen.github.io/wasm-bindgen/examples/closures.html", text("The wasm-bindgen Guide")),
            text("."),
        ],
        h3_id!("using-gloo-concise", text("Using "), code("gloo"), text(" (concise)")),
        p![
            text("The easier way is with "),
            code("gloo"),
            text(", more specifically "),
            link!("https://docs.rs/gloo-events/0.1.1/gloo_events/index.html", code("gloo_events")),
            text(" which is an abstraction for "),
            code("web-sys"),
            text(", "),
            code("wasm-bindgen"),
            text("."),
        ],
        p![
            code("gloo_events"),
            text(" has the "),
            code("EventListener"),
            text(" type which can be used to create and store the event listener."),
        ],
        code_block_title("toml", "Cargo.toml", r#"[dependencies]
gloo-events = "0.1""#),
        code_block("rust", r#"use web_sys::HtmlElement;
use yew::prelude::*;

use gloo::events::EventListener;

#[function_component]
fn MyComponent() -> Html {
let div_node_ref = use_node_ref();

use_effect_with_deps(
{
    let div_node_ref = div_node_ref.clone();

    move |_| {
        let mut custard_listener = None;

        if let Some(element) = div_node_ref.cast::<HtmlElement>() {
            // Create your Callback as you normally would
            let oncustard = Callback::from(move |_: Event| {
                // do something about custard..
            });

            // Create a Closure from a Box<dyn Fn> - this has to be 'static
            let listener = EventListener::new(
                &element,
                "custard",
                move |e| oncustard.emit(e.clone())
            );

            custard_listener = Some(listener);
        }

        move || drop(custard_listener)
    }
},
div_node_ref.clone()
);

html! {
<div ref={div_node_ref} id="my-div"></div>
}
}"#),
        p![
            text("For more information on "),
            code("EventListener"),
            text(", see the "),
            link!("https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html", text("gloo_events docs.rs")),
            text("."),
        ],
        h2_id!("available-events", text("Full list of available events")),
        table(
            vec![
                vec![text("Event listener name")],
                vec![code("web_sys"), text(" Event Type")],
            ],
            vec![
                vec![vec![code("onabort")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onauxclick")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onblur")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent"))]],
                vec![vec![code("oncancel")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("oncanplay")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("oncanplaythrough")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onchange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onclick")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onclose")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("oncontextmenu")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("oncuechange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("ondblclick")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("ondrag")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondragend")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondragenter")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondragexit")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondragleave")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondragover")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondragstart")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondrop")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", text("DragEvent"))]],
                vec![vec![code("ondurationchange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onemptied")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onended")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onerror")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onfocus")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent"))]],
                vec![vec![code("onfocusin")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent"))]],
                vec![vec![code("onfocusout")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", text("FocusEvent"))]],
                vec![vec![code("onformdata")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("oninput")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.InputEvent.html", text("InputEvent"))]],
                vec![vec![code("oninvalid")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onkeydown")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", text("KeyboardEvent"))]],
                vec![vec![code("onkeypress")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", text("KeyboardEvent"))]],
                vec![vec![code("onkeyup")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", text("KeyboardEvent"))]],
                vec![vec![code("onload")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onloadeddata")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onloadedmetadata")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onloadstart")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", text("ProgressEvent"))]],
                vec![vec![code("onmousedown")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onmouseenter")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onmouseleave")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onmousemove")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onmouseout")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onmouseover")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onmouseup")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", text("MouseEvent"))]],
                vec![vec![code("onpause")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onplay")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onplaying")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onprogress")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", text("ProgressEvent"))]],
                vec![vec![code("onratechange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onreset")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onresize")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onscroll")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onsecuritypolicyviolation")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onseeked")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onseeking")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onselect")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onslotchange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onstalled")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onsubmit")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.SubmitEvent.html", text("SubmitEvent"))]],
                vec![vec![code("onsuspend")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("ontimeupdate")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("ontoggle")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onvolumechange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onwaiting")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onwheel")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html", text("WheelEvent"))]],
                vec![vec![code("oncopy")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("oncut")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onpaste")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onanimationcancel")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent"))]],
                vec![vec![code("onanimationend")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent"))]],
                vec![vec![code("onanimationiteration")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent"))]],
                vec![vec![code("onanimationstart")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", text("AnimationEvent"))]],
                vec![vec![code("ongotpointercapture")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onloadend")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", text("ProgressEvent"))]],
                vec![vec![code("onlostpointercapture")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointercancel")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointerdown")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointerenter")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointerleave")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointerlockchange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onpointerlockerror")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onpointermove")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointerout")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointerover")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onpointerup")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", text("PointerEvent"))]],
                vec![vec![code("onselectionchange")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onselectstart")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("onshow")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.Event.html", text("Event"))]],
                vec![vec![code("ontouchcancel")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent"))]],
                vec![vec![code("ontouchend")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent"))]],
                vec![vec![code("ontouchmove")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent"))]],
                vec![vec![code("ontouchstart")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", text("TouchEvent"))]],
                vec![vec![code("ontransitioncancel")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent"))]],
                vec![vec![code("ontransitionend")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent"))]],
                vec![vec![code("ontransitionrun")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent"))]],
                vec![vec![code("ontransitionstart")], vec![link!("https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", text("TransitionEvent"))]],
            ],
        ),
    ])
);
