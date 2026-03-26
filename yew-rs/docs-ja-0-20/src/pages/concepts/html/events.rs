crate::doc_page!("Events", "/ja/docs/concepts/html/events",
    Content::new(vec![
        h2!["Introduction"],
        p![
            "Yew integrates with the ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/", code("web-sys")],
            " crate and uses the events from that crate. The ",
            link!["#event-types", "table below"],
            " lists all of the ",
            code("web-sys"),
            " events that are accepted in the ",
            code("html!"),
            " macro.",
        ],
        p![
            "You can still add a ",
            doc_link![crate::pages::concepts::function_components::callbacks, code("Callback")],
            " for an event that is not listed in the table below, see ",
            link!["#manual-event-listener", "Manual event listener"],
            ".",
        ],
        h2!["Event Types"],
        admonition![AdmonitionType::Tip, None,
            p![
                "All the event types mentioned in the following table are re-exported under ",
                code("yew::events"),
                ". Using the types from ",
                code("yew::events"),
                " makes it easier to ensure version compatibility than \
                  if you were to manually include ",
                code("web-sys"),
                " as a dependency in your crate because you won't \
                  end up using a version which conflicts with the version that Yew specifies.",
            ],
        ],
        p![
            "The event listener name is the expected name when adding an event ",
            code("Callback"),
            " in the ",
            code("html"),
            " macro:",
        ],
        code_block("rust", r#"use yew::prelude::*;

html! {
<button onclick={Callback::from(|_| ())}>
//      ^^^^^^^ event listener name
{ "Click me!" }
</button>
};"#),
        p![
            "The event name is the listener without the \"on\" prefix, therefore, the ",
            code("onclick"),
            " event listener listens for ",
            code("click"),
            " events. See the end of this page for a ",
            link!["#event-types", "full list of available events"],
            " with their types.",
        ],
        h2_id!["event-bubbling", "Event bubbling"],
        p![
            "Events dispatched by Yew follow the virtual DOM hierarchy when bubbling up to listeners. Currently, only the bubbling phase \
              is supported for listeners. Note that the virtual DOM hierarchy is most often, but not always, identical to the actual \
              DOM hierarchy. The distinction is important when working with ",
            doc_link![crate::pages::advanced_topics::portals, "portals"],
            " and other more advanced techniques. The intuition for well implemented components should be that events bubble from children \
              to parents, so that the hierarchy in your coded ",
            code("html!"),
            " is the one observed by event handlers.",
        ],
        p![
            "If you are not interested in event bubbling, you can turn it off by calling",
        ],
        code_block("rust", r#"yew::set_event_bubbling(false);"#),
        p![
            italic!["before"],
            " starting your app. This speeds up event handling, but some components may break from not receiving events they expect. \
              Use this with care!",
        ],
        h2!["Event delegation"],
        p![
            "It can be surprising that event listeners are ",
            italic!["not"],
            " directly registered on the element where they are rendered. Instead, events \
              are delegated from the subtree root of the Yew app. Still, events are delivered in their native form, and no synthetic \
              form is created. This can lead to mismatches between the event you'd expect in html listeners and those showing up in Yew.",
        ],
        ul![
            li![
                link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.current_target", code("Event::current_target")],
                " points to the Yew subtree root instead of the element the listener is added on. Use ",
                doc_link![crate::pages::concepts::function_components::node_refs, code("NodeRef")],
                " if you want access to the underlying ",
                code("HtmlElement"),
                ".",
            ],
            li![
                link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.event_phase", code("Event::event_phase")],
                " is always ",
                code("Event::CAPTURING_PHASE"),
                ". Internally, the event will behave as if it was in the bubbling \
                  phase, the event propagation is replayed and the event ",
                link!["#event-bubbling", "bubbles up"],
                ", i.e. event listeners higher up in \
                  the virtual DOM will trigger after event listeners below them. Currently, capturing listeners are not supported by Yew. \
                  This also means that events registered by Yew will usually fire before other event listeners.",
            ],
        ],
        h2!["Typed event target"],
        admonition![AdmonitionType::Caution, None,
            p![
                "In this section ",
                bold!["target"],
                " (",
                link!["https://developer.mozilla.org/en-US/docs/Web/API/Event/target", code("Event.target")],
                ") is always referring to the element at which the event was dispatched from.",
            ],
            p![
                "This will ",
                bold!["not"],
                " always be the element at which the ",
                code("Callback"),
                " is placed.",
            ],
        ],
        p![
            "In event ",
            code("Callback"),
            "s you may want to get the target of that event. For example, the ",
            code("change"),
            " event gives no information but is used to notify that something has changed.",
        ],
        p![
            "In Yew getting the target element in the correct type can be done in a few ways and we will go through \
              them here. Calling ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target", code("web_sys::Event::target")],
            " on an event returns an optional ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html", code("web_sys::EventTarget")],
            " type, which might not seem very useful when you want to know the value of your input element.",
        ],
        p![
            "In all the approaches below we are going to tackle the same problem, so it's clear where the approach \
              differs opposed to the problem at hand.",
        ],
        p![bold!["The Problem:"]],
        p![
            "We have an ",
            code("onchange"),
            " ",
            code("Callback"),
            " on my ",
            code("<input>"),
            " element and each time it is invoked we want to send \
              an update ",
            code("Msg"),
            " to our component.",
        ],
        p![
            "Our ",
            code("Msg"),
            " enum looks like this:",
        ],
        code_block("rust", r#"pub enum Msg {
InputValue(String),
}"#),
        h3!["Using JsCast"],
        p![
            "The ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html", code("wasm-bindgen")],
            " crate has a useful trait; ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", code("JsCast")],
            ", which allows us to hop and skip our way to the type we want, as long as it implements ",
            code("JsCast"),
            ". We can do this cautiously, which involves some runtime checks and failure types like ",
            code("Option"),
            " and ",
            code("Result"),
            ", or we can do it dangerously.",
        ],
        code_block("toml", r#"[dependencies]
# need wasm-bindgen for JsCast
wasm-bindgen = "0.2""#),
        code_block("rust", r#"//highlight-next-line
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component(MyComponent)]
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
            "The methods from ",
            code("JsCast"),
            " are ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into", code("dyn_into")],
            " and ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into", code("unchecked_into")],
            " and they allowed us to go from ",
            code("EventTarget"),
            " to ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html", code("HtmlInputElement")],
            ". The ",
            code("dyn_into"),
            " method is cautious because at runtime it will check whether the type is actually a ",
            code("HtmlInputElement"),
            " and if not return an ",
            code("Err(JsValue)"),
            ", the ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html", code("JsValue")],
            " is a catch-all type and is essentially giving you back the object to try again.",
        ],
        p![
            "At this point you might be thinking... when is the dangerous version ok to use? In the case above it \
              is safe",
            sup!["1"],
            " as we've set the ",
            code("Callback"),
            " on to an element with no children so the target can only be that same element.",
        ],
        p![
            italic![
                sup!["1"],
                " As safe as anything can be when JS land is involved.",
            ],
        ],
        h3!["Using TargetCast"],
        p![
            bold!["It is highly recommended to read "],
            link!["#using-jscast", "Using JsCast"],
            bold![" first!"],
        ],
        admonition![AdmonitionType::Note, None,
            p![
                code("TargetCast"),
                " was designed to feel very similar to ",
                code("JsCast"),
                " - this is to allow new users to get a feel \
                  for the behaviour of ",
                code("JsCast"),
                " but with the smaller scope of events and their targets.",
            ],
            p![
                code("TargetCast"),
                " vs ",
                code("JsCast"),
                " is purely preference, you will find that ",
                code("TargetCast"),
                " implements something similar to what you would using ",
                code("JsCast"),
                ".",
            ],
        ],
        p![
            "The ",
            code("TargetCast"),
            " trait is built on top of ",
            code("JsCast"),
            " and is specialized towards getting typed event targets from events.",
        ],
        p![
            code("TargetCast"),
            " comes with Yew so no need to add a dependency in order to use the trait methods on events \
              but it works in a very similar way to ",
            code("JsCast"),
            ".",
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(MyComponent)]
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
            "If you followed the advice above and read about ",
            code("JsCast"),
            ", you can probably see that ",
            code("TargetCast::target_dyn_into"),
            " feels similar to ",
            code("JsCast::dyn_into"),
            " but specifically does the cast on the target of the event. ",
            code("TargetCast::target_unchecked_into"),
            " is similar to ",
            code("JsCast::unchecked_into"),
            ", and as such all the same warnings above ",
            code("JsCast"),
            " apply to ",
            code("TargetCast"),
            ".",
        ],
        h3!["Using NodeRef"],
        p![
            doc_link![crate::pages::concepts::function_components::node_refs, code("NodeRef")],
            " can be used instead of querying the event given to a ",
            code("Callback"),
            ".",
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(MyComponent)]
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
            "Using ",
            code("NodeRef"),
            ", you can ignore the event and use the ",
            code("NodeRef::cast"),
            " method to get an ",
            code("Option<HtmlInputElement>"),
            " - this is optional as calling ",
            code("cast"),
            " before the ",
            code("NodeRef"),
            " has been set, or when the type doesn't match will return ",
            code("None"),
            ".",
        ],
        p![
            "You might also see by using ",
            code("NodeRef"),
            " we don't have to send the ",
            code("String"),
            " back into state as we always have access to ",
            code("input_node_ref"),
            " - so we could do the following:",
        ],
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(MyComponent)]
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
            "Which approach you take depends on your component and your preferences, there is no blessed way per se.",
        ],
        h2_id!["manual-event-listener", "Manual event listener"],
        p![
            "You may want to listen to an event that is not supported by Yew's ",
            code("html"),
            " macro, see the ",
            link!["#event-types", "supported events listed here"],
            ".",
        ],
        p![
            "In order to add an event listener to one of elements manually we need the help of ",
            doc_link![crate::pages::concepts::function_components::node_refs, code("NodeRef")],
            " so that in ",
            code("use_effect_with_deps"),
            " we can add a listener using the ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/index.html", code("web-sys")],
            " and ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html", "wasm-bindgen"],
            " API.",
        ],
        p![
            "The examples below are going to show adding listeners for the made-up ",
            code("custard"),
            " event. All events either unsupported by yew or custom can be represented as a ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html", code("web_sys::Event")],
            ". If you need to access a specific method or field on a custom / unsupported event then you can use the \
              methods of ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html", code("JsCast")],
            " in order to convert to the type required.",
        ],
        h3!["Using Closure (verbose)"],
        p![
            "Using the ",
            code("web-sys"),
            " and ",
            code("wasm-bindgen"),
            " API's directly for this can be a bit painful... so brace \
              yourself (",
            link!["#using-gloo-concise", "there is a more concise way thanks to gloo"],
            ").",
        ],
        code_block("rust", r#"use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(MyComponent)]
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
            "For more information on ",
            code("Closures"),
            ", see ",
            link!["https://wasm-bindgen.github.io/wasm-bindgen/examples/closures.html", "The wasm-bindgen Guide"],
            ".",
        ],
        h3_id!["using-gloo-concise", "Using gloo (concise)"],
        p![
            "The easier way is with ",
            code("gloo"),
            ", more specifically ",
            link!["https://docs.rs/gloo-events/0.1.1/gloo_events/index.html", code("gloo_events")],
            " which is an abstraction for ",
            code("web-sys"),
            ", ",
            code("wasm-bindgen"),
            ".",
        ],
        p![
            code("gloo_events"),
            " has the ",
            code("EventListener"),
            " type which can be used to create and store the event listener.",
        ],
        code_block("toml", r#"[dependencies]
gloo-events = "0.1""#),
        code_block("rust", r#"use web_sys::HtmlElement;
use yew::prelude::*;

use gloo::events::EventListener;

#[function_component(MyComponent)]
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
            "For more information on ",
            code("EventListener"),
            ", see the ",
            link!["https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html", "gloo_events docs.rs"],
            ".",
        ],
        h2_id!["available-events", "Full list of available events"],
        table(
            vec![
                vec!["Event listener name".into()],
                vec![code("web_sys"), " Event Type".into()],
            ],
            vec![
                vec![vec![code("onabort")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onauxclick")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onblur")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("oncancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncanplay")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncanplaythrough")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onclick")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onclose")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncontextmenu")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("oncuechange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ondblclick")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("ondrag")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragenter")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragexit")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragleave")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragover")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondragstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondrop")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.DragEvent.html", "DragEvent"]]],
                vec![vec![code("ondurationchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onemptied")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onended")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onerror")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onfocus")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("onfocusin")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("onfocusout")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.FocusEvent.html", "FocusEvent"]]],
                vec![vec![code("onformdata")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oninput")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.InputEvent.html", "InputEvent"]]],
                vec![vec![code("oninvalid")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onkeydown")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"]]],
                vec![vec![code("onkeypress")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"]]],
                vec![vec![code("onkeyup")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.KeyboardEvent.html", "KeyboardEvent"]]],
                vec![vec![code("onload")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onloadeddata")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onloadedmetadata")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onloadstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"]]],
                vec![vec![code("onmousedown")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseenter")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseleave")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmousemove")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseout")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseover")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onmouseup")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.MouseEvent.html", "MouseEvent"]]],
                vec![vec![code("onpause")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onplay")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onplaying")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onprogress")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"]]],
                vec![vec![code("onratechange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onreset")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onresize")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onscroll")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onsecuritypolicyviolation")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onseeked")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onseeking")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onselect")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onslotchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onstalled")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onsubmit")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.SubmitEvent.html", "SubmitEvent"]]],
                vec![vec![code("onsuspend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ontimeupdate")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ontoggle")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onvolumechange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onwaiting")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onwheel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.WheelEvent.html", "WheelEvent"]]],
                vec![vec![code("oncopy")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("oncut")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onpaste")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onanimationcancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("onanimationend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("onanimationiteration")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("onanimationstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.AnimationEvent.html", "AnimationEvent"]]],
                vec![vec![code("ongotpointercapture")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onloadend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.ProgressEvent.html", "ProgressEvent"]]],
                vec![vec![code("onlostpointercapture")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointercancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerdown")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerenter")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerleave")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerlockchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onpointerlockerror")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onpointermove")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerout")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerover")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onpointerup")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.PointerEvent.html", "PointerEvent"]]],
                vec![vec![code("onselectionchange")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onselectstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("onshow")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.Event.html", "Event"]]],
                vec![vec![code("ontouchcancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontouchend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontouchmove")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontouchstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TouchEvent.html", "TouchEvent"]]],
                vec![vec![code("ontransitioncancel")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
                vec![vec![code("ontransitionend")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
                vec![vec![code("ontransitionrun")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
                vec![vec![code("ontransitionstart")], vec![link!["https://docs.rs/web-sys/latest/web_sys/struct.TransitionEvent.html", "TransitionEvent"]]],
            ],
        ),
    ])
);
