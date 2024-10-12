use yew::prelude::*;

struct NotToString;

fn compile_fail() {
    // missing closing tag
    html! { <div> };
    html! { <div><div> };
    html! { <div><div></div> };

    // missing opening tag
    html! { </div> };
    html! { <div></span></div> };
    html! { <img /></img> };

    // tag mismatch
    html! { <div></span> };
    html! { <tag-a></tag-b> };

    // multiple root
    html! { <div></div><div></div> };
    // invalid child content
    html! { <div>Invalid</div> };

    // same attribute specified multiple times (tests for attributes with special treatment)
    html! { <input attr=1 attr=2 /> };
    html! { <input value="123" value="456" /> };
    html! { <input kind="checkbox" kind="submit" /> };
    html! { <input checked=true checked=false /> };
    html! { <input disabled=true disabled=false /> };
    html! { <option selected=true selected=false /> };
    html! { <div class="first" class="second" /> };
    html! { <input ref={()} ref={()} /> };

    // boolean attribute type mismatch
    html! { <input checked=1 /> };
    html! { <input checked={Some(false)} /> };
    html! { <input disabled=1 /> };
    html! { <input disabled={Some(true)} /> };
    html! { <option selected=1 /> };

    // normal attribute type mismatch
    html! { <input type={()} /> };
    html! { <input value={()} /> };
    html! { <a href={()} /> };
    html! { <input string={NotToString} /> };
    html! { <a media={Some(NotToString)} /> };
    html! { <a href={Some(5)} /> };

    // listener type mismatch
    html! { <input onclick=1 /> };
    html! { <input onclick={Callback::from(|a: String| ())} /> };
    html! { <input onfocus={Some(5)} /> };

    // NodeRef type mismatch
    html! { <input ref={()} /> };
    html! { <input ref={Some(NodeRef::default())} /> };
    html! { <input onclick={Callback::from(|a: String| ())} /> };

    html! { <input string={NotToString} /> };

    html! { <input ref={()} /> };
    html! { <input ref={()} ref={()} /> };

    // void element with children
    html! { <input type="text"></input> };
    // make sure that capitalization doesn't matter for the void children check
    html! { <iNpUt type="text"></iNpUt> };

    // no tag name
    html! { <@></@> };
    html! { <@/> };

    // invalid closing tag
    html! { <@{"test"}></@{"test"}> };
    // type mismatch
    html! { <@{55}></@> };

    // Missing curly braces
    html! { <input ref=() /> };
    html! { <input ref=() ref=() /> };
    html! { <input onfocus=Some(5) /> };
    html! { <input string=NotToString /> };
    html! { <a media=Some(NotToString) /> };
    html! { <a href=Some(5) /> };
    html! { <input type=() /> };
    html! { <input value=() /> };
    html! { <input string=NotToString /> };
}

fn main() {}
