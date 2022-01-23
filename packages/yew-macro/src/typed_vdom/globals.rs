use syn::parse_quote;

use crate::typed_vdom::{AttributePropDefinition, ListenerPropDefinition};

pub fn global_attributes() -> [AttributePropDefinition; 17] {
    [
        AttributePropDefinition::new(
            parse_quote! { autocapitalize },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { contextmenu },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { contenteditable },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { slot },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { spellcheck },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { class },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { title },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { itemprop },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { accesskey },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { lang },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { id },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { translate },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { draggable },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { style },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { dir },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { tabindex },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
        AttributePropDefinition::new(
            parse_quote! { hidden },
            parse_quote! { ::yew::virtual_dom::AttrValue },
        ),
    ]
}

pub fn listeners() -> [ListenerPropDefinition; 96] {
    [
        ListenerPropDefinition::new(parse_quote! { abort }),
        ListenerPropDefinition::new(parse_quote! { cancel }),
        ListenerPropDefinition::new(parse_quote! { canplay }),
        ListenerPropDefinition::new(parse_quote! { canplaythrough }),
        ListenerPropDefinition::new(parse_quote! { close }),
        ListenerPropDefinition::new(parse_quote! { cuechange }),
        ListenerPropDefinition::new(parse_quote! { durationchange }),
        ListenerPropDefinition::new(parse_quote! { emptied }),
        ListenerPropDefinition::new(parse_quote! { ended }),
        ListenerPropDefinition::new(parse_quote! { error }),
        ListenerPropDefinition::new(parse_quote! { formdata }),
        ListenerPropDefinition::new(parse_quote! { invalid }),
        ListenerPropDefinition::new(parse_quote! { load }),
        ListenerPropDefinition::new(parse_quote! { loadeddata }),
        ListenerPropDefinition::new(parse_quote! { loadedmetadata }),
        ListenerPropDefinition::new(parse_quote! { pause }),
        ListenerPropDefinition::new(parse_quote! { play }),
        ListenerPropDefinition::new(parse_quote! { playing }),
        ListenerPropDefinition::new(parse_quote! { ratechange }),
        ListenerPropDefinition::new(parse_quote! { reset }),
        ListenerPropDefinition::new(parse_quote! { resize }),
        ListenerPropDefinition::new(parse_quote! { securitypolicyviolation }),
        ListenerPropDefinition::new(parse_quote! { seeked }),
        ListenerPropDefinition::new(parse_quote! { seeking }),
        ListenerPropDefinition::new(parse_quote! { select }),
        ListenerPropDefinition::new(parse_quote! { slotchange }),
        ListenerPropDefinition::new(parse_quote! { stalled }),
        ListenerPropDefinition::new(parse_quote! { suspend }),
        ListenerPropDefinition::new(parse_quote! { timeupdate }),
        ListenerPropDefinition::new(parse_quote! { toggle }),
        ListenerPropDefinition::new(parse_quote! { volumechange }),
        ListenerPropDefinition::new(parse_quote! { waiting }),
        ListenerPropDefinition::new(parse_quote! { change }),
        ListenerPropDefinition::new(parse_quote! { copy }),
        ListenerPropDefinition::new(parse_quote! { cut }),
        ListenerPropDefinition::new(parse_quote! { paste }),
        ListenerPropDefinition::new(parse_quote! { pointerlockchange }),
        ListenerPropDefinition::new(parse_quote! { pointerlockerror }),
        ListenerPropDefinition::new(parse_quote! { selectionchange }),
        ListenerPropDefinition::new(parse_quote! { selectstart }),
        ListenerPropDefinition::new(parse_quote! { show }),
        ListenerPropDefinition::new_with_ty(parse_quote! { auxclick }, parse_quote! { MouseEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { click }, parse_quote! { MouseEvent }),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { contextmenu },
            parse_quote! { MouseEvent },
        ),
        ListenerPropDefinition::new_with_ty(parse_quote! { dblclick }, parse_quote! { MouseEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { drag }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { dragend }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { dragenter }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { dragexit }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { dragleave }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { dragover }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { dragstart }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { drop }, parse_quote! { DragEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { blur }, parse_quote! { FocusEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { focus }, parse_quote! { FocusEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { focusin }, parse_quote! { FocusEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { focusout }, parse_quote! { FocusEvent }),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { keydown },
            parse_quote! { KeyboardEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { keypress },
            parse_quote! { KeyboardEvent },
        ),
        ListenerPropDefinition::new_with_ty(parse_quote! { keyup }, parse_quote! { KeyboardEvent }),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { loadstart },
            parse_quote! { ProgressEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { progress },
            parse_quote! { ProgressEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { loadend },
            parse_quote! { ProgressEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { mousedown },
            parse_quote! { MouseEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { mouseenter },
            parse_quote! { MouseEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { mouseleave },
            parse_quote! { MouseEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { mousemove },
            parse_quote! { MouseEvent },
        ),
        ListenerPropDefinition::new_with_ty(parse_quote! { mouseout }, parse_quote! { MouseEvent }),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { mouseover },
            parse_quote! { MouseEvent },
        ),
        ListenerPropDefinition::new_with_ty(parse_quote! { mouseup }, parse_quote! { MouseEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { wheel }, parse_quote! { WheelEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { input }, parse_quote! { InputEvent }),
        ListenerPropDefinition::new_with_ty(parse_quote! { submit }, parse_quote! { FocusEvent }),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { animationcancel },
            parse_quote! { AnimationEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { animationend },
            parse_quote! { AnimationEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { animationiteration },
            parse_quote! { AnimationEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { animationstart },
            parse_quote! { AnimationEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { gotpointercapture },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { lostpointercapture },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointercancel },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointerdown },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointerenter },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointerleave },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointermove },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointerout },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointerover },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { pointerup },
            parse_quote! { PointerEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { touchcancel },
            parse_quote! { TouchEvent },
        ),
        ListenerPropDefinition::new_with_ty(parse_quote! { touchend }, parse_quote! { TouchEvent }),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { transitioncancel },
            parse_quote! { TransitionEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { transitionend },
            parse_quote! { TransitionEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { transitionrun },
            parse_quote! { TransitionEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { transitionstart },
            parse_quote! { TransitionEvent },
        ),
        ListenerPropDefinition::new(parse_quote! { scroll }),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { touchmove },
            parse_quote! { TouchEvent },
        ),
        ListenerPropDefinition::new_with_ty(
            parse_quote! { touchstart },
            parse_quote! { TouchEvent },
        ),
    ]
}
