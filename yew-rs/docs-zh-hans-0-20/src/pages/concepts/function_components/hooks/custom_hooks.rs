crate::doc_page!(
    "\u{81ea}\u{5b9a}\u{4e49}\u{94a9}\u{5b50}\u{ff08}Custom Hooks\u{ff09}",
    "/zh-Hans/docs/concepts/function-components/hooks/custom-hooks",
    Content::new(vec![
        h2(vec![text(
            "\u{5b9a}\u{4e49}\u{81ea}\u{5b9a}\u{4e49}\u{94a9}\u{5b50}"
        )]),
        p(vec![text(
            "\u{7ec4}\u{4ef6}\u{4e2d}\u{4e0e}\u{72b6}\u{6001}\u{6709}\u{5173}\u{7684}\u{903b}\\
             u{8f91}\u{53ef}\u{4ee5}\u{901a}\u{8fc7}\u{521b}\u{5efa}\u{81ea}\u{5b9a}\u{4e49} Hooks \
             \u{63d0}\u{53d6}\u{5230}\u{51fd}\u{6570}\u{4e2d}\u{3002}"
        )]),
        p(vec![text(
            "\u{5047}\u{8bbe}\u{6211}\u{4eec}\u{6709}\u{4e00}\u{4e2a}\u{7ec4}\u{4ef6}\u{ff0c}\\
             u{5b83}\u{8ba2}\u{9605}\u{4e86}\u{4e00}\u{4e2a}\u{4ee3}\u{7406}\u{ff08}agent\u{ff09}\\
             u{5e76}\u{4e14}\u{4f1a}\u{663e}\u{793a}\u{53d1}\u{9001}\u{7ed9}\u{5b83}\u{7684}\\
             u{6d88}\u{606f}\u{3002}"
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use gloo::events::EventListener;
use gloo::utils::window;
use std::mem::drop;


#[function_component(ShowStorageChanged)]
pub fn show_storage_changed() -> Html {
    let state_storage_changed = use_state(|| false);

    {
        let state_storage_changed = state_storage_changed.clone();
        use_effect(|| {
            let listener = EventListener::new(&window(), "storage", move |_| state_storage_changed.set(true));

            move || { drop(listener); }
        });
    }

    html! { <div>{"Storage Event Fired: "}{*state_storage_changed}</div> }
}"#
        ),
        p(vec![text(
            "\u{8fd9}\u{6bb5}\u{4ee3}\u{7801}\u{6709}\u{4e00}\u{4e2a}\u{95ee}\u{9898}\u{ff1a}\\
             u{903b}\u{8f91}\u{4e0d}\u{80fd}\u{88ab}\u{53e6}\u{4e00}\u{4e2a}\u{7ec4}\u{4ef6}\\
             u{91cd}\u{7528}\u{3002}\u{5982}\u{679c}\u{6211}\u{4eec}\u{6784}\u{5efa}\u{53e6}\\
             u{4e00}\u{4e2a}\u{8ddf}\u{8e2a}\u{6d88}\u{606f}\u{7684}\u{7ec4}\u{4ef6}\u{ff0c}\\
             u{6211}\u{4eec}\u{53ef}\u{4ee5}\u{5c06}\u{903b}\u{8f91}\u{79fb}\u{52a8}\u{5230}\\
             u{81ea}\u{5b9a}\u{4e49}\u{94a9}\u{5b50}\u{4e2d}\u{ff0c}\u{800c}\u{4e0d}\u{662f}\\
             u{590d}\u{5236}\u{4ee3}\u{7801}\u{3002}"
        )]),
        p(vec![
            text(
                "\u{6211}\u{4eec}\u{5c06}\u{9996}\u{5148}\u{521b}\u{5efa}\u{4e00}\u{4e2a}\u{540d}\\
                 \
                 u{4e3a}"
            ),
            code("use_subscribe"),
            text("\u{7684}\u{65b0}\u{51fd}\u{6570}\u{3002} "),
            code("use_"),
            text(
                "\u{524d}\u{7f00}\u{901a}\u{5e38}\u{8868}\u{793a}\u{6b64}\u{51fd}\u{6570}\u{662f}\\
                 \
                 u{4e00}\u{4e2a}\u{94a9}\u{5b50}\u{3002}\u{8fd9}\u{4e2a}\u{51fd}\u{6570}\u{5c06}\\
                 u{4e0d}\u{63a5}\u{53d7}\u{4efb}\u{4f55}\u{53c2}\u{6570}\u{5e76}\u{8fd4}\u{56de}"
            ),
            code("Rc<RefCell<Vec<String>>>"),
            text(" \u{3002}"),
        ]),
        code_block(
            "rust",
            r#"use web_sys::{Event, EventTarget};
use std::borrow::Cow;
use gloo::events::EventListener;
use yew::prelude::*;

#[hook]
pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(&Event) + 'static,
{
    todo!()
}"#
        ),
        p(vec![
            text("\u{94a9}\u{5b50}\u{7684}\u{903b}\u{8f91}\u{5728}"),
            code("use_hook"),
            text("\u{7684}\u{56de}\u{8c03}\u{4e2d}\u{3002} "),
            code("use_hook"),
            text(
                "\u{6307}\u{7684}\u{662f}\u{81ea}\u{5b9a}\u{4e49} Hook \
                 \u{7684}\u{5904}\u{7406}\u{51fd}\u{6570}\u{3002}\u{5b83}\u{63a5}\u{53d7} 2 \
                 \u{4e2a}\u{53c2}\u{6570}\u{ff1a} "
            ),
            code("hook_runner"),
            text("\u{548c}"),
            code("initial_state_producer"),
            text(" \u{3002}"),
        ]),
        p(vec![
            code("hook_runner"),
            text(
                "\u{4e2d}\u{5305}\u{542b}\u{4e86}\u{6240}\u{6709}\u{94a9}\u{5b50}\u{7684}\u{903b}\\
                 \
                 u{8f91}\u{ff0c}\u{5b83}\u{7684}\u{56de}\u{8c03}\u{7684}\u{8fd4}\u{56de}\u{503c}\\
                 u{53c8}\u{4f1a}\u{88ab}"
            ),
            code("use_hook"),
            text("\u{8fd4}\u{56de}\u{3002} "),
            code("hook_runner"),
            text(
                "\u{9700}\u{8981} 2 \
                 \u{4e2a}\u{53c2}\u{6570}\u{ff1a}\u{5206}\u{522b}\u{662f}\u{5bf9}\u{94a9}\u{5b50}\\
                 \
                 u{548c}"
            ),
            code("hook_callback"),
            text(
                "\u{5b83}\u{4eec}\u{4e24}\u{4e2a}\u{7684}\u{5185}\u{90e8}\u{72b6}\u{6001}\u{7684}\\
                 \
                 u{53ef}\u{53d8}\u{5f15}\u{7528}\u{3002} \u{800c}"
            ),
            code("hook_callback"),
            text(
                "\u{540c}\u{6837}\u{4e5f}\u{8981} 2 \
                 \u{4e2a}\u{53c2}\u{6570}\u{ff1a}\u{4e00}\u{4e2a}\u{56de}\u{8c03}\u{548c}\u{4e00}\\
                 \
                 u{4e2a} bool\u{ff0c}\u{56de}\u{8c03}\u{63a5}\u{53d7}"
            ),
            code("internal_state"),
            text(
                " \u{ff0c}\u{4e5f}\u{5c31}\u{662f}\u{5bf9}\u{5185}\u{90e8}\u{72b6}\u{6001}\\
                 u{5b9e}\u{4f8b}\u{7684}\u{53ef}\u{53d8}\u{5f15}\u{7528}\u{ff0c}\u{5e76}\u{4e14}\\
                 u{4f1a}\u{8c03}\u{6267}\u{884c}\u{5b9e}\u{9645}\u{7684}\u{66f4}\u{6539}\u{ff0c}\\
                 u{8fd8}\u{4f1a}\u{8fd4}\u{56de}\u{8868}\u{793a}"
            ),
            code("ShouldRender"),
            text(
                "\u{7684}\u{5e03}\u{5c14}\u{503c}\u{ff0c}\u{7b2c}\u{4e8c}\u{4e2a}\u{53c2}\u{6570} \
                 bool \u{7684}\u{7528}\u{5904}\u{662f}\u{6307}\u{793a}\u{5b83}\u{662f}\u{5426}\\
                 u{5728}\u{7ec4}\u{4ef6}\u{6e32}\u{67d3}\u{540e}\u{8fd0}\u{884c}\u{3002}"
            ),
            code("use_hook"),
            text("\u{7684}\u{7b2c}\u{4e8c}\u{4e2a}\u{53c2}\u{6570}"),
            code("initial_state_producer"),
            text(
                "\u{63a5}\u{53d7}\u{7528}\u{4e8e}\u{521b}\u{5efa}\u{5185}\u{90e8}\u{72b6}\u{6001}\\
                 \
                 u{5b9e}\u{4f8b}\u{7684}\u{56de}\u{8c03}\u{3002}\u{8fd9}\u{91cc}\u{8bf4}\u{7684}\\
                 u{5185}\u{90e8}\u{72b6}\u{6001}\u{6307}\u{7684}\u{662f}\u{4e00}\u{4e2a}\u{5b9e}\\
                 u{73b0}\u{4e86}"
            ),
            code("Hook"),
            text(" trait \u{7684}\u{7ed3}\u{6784}\u{4f53}\u{3002}"),
        ]),
        p(vec![
            text("\u{73b0}\u{5728}\u{8ba9}\u{6211}\u{4eec}\u{4e3a}"),
            code("use_subscribe"),
            text(
                "\u{94a9}\u{5b50}\u{521b}\u{5efa}\u{72b6}\u{6001}\u{ff08}state \
                 struct\u{ff09}\u{3002}"
            ),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;
use web_sys::{Event, EventTarget};
use std::borrow::Cow;
use std::rc::Rc;
use gloo::events::EventListener;

#[hook]
pub fn use_event<E, F>(target: &EventTarget, event_type: E, callback: F)
where
    E: Into<Cow<'static, str>>,
    F: Fn(Event) + 'static,
{
    #[derive(PartialEq, Clone)]
    struct EventDependents {
        target: EventTarget,
        event_type: Cow<'static, str>,
        callback: Callback<Event>,
    }

    let deps = EventDependents {
        target: target.clone(),
        event_type: event_type.into(),
        callback: Callback::from(callback),
    };

    use_effect_with_deps(
        |deps| {
            let EventDependents {
                target,
                event_type,
                callback,
            } = deps.clone();

            let listener = EventListener::new(&target, event_type, move |e| {
                callback.emit(e.clone());
            });

            move || {
                drop(listener);
            }
        },
        deps,
    );
}"#
        ),
        p(vec![
            text("\u{63a5}\u{4e0b}\u{6765}\u{6211}\u{4eec}\u{4e3a}"),
            code("use_subscribe"),
            text("\u{6dfb}\u{52a0}\u{5b9e}\u{9645}\u{903b}\u{8f91}\u{3002}"),
        ]),
        code_block(
            "rust",
            r#"fn use_subscribe() -> Rc<RefCell<Vec<String>>> {
    use_hook(
        // hook's handler. all the logic goes in here
        |state: &mut UseSubscribeState, hook_callback| {
            // calling other Hooks inside a hook
            use_effect(move || {
                let producer = EventBus::bridge(Callback::from(move |msg| {
                    hook_callback(
                        // where the mutations of state are performed
                        |state| {
                            (*state.messages).borrow_mut().deref_mut().push(msg);
                            true // should re-render
                        }, false // run post-render
                    )
                }));

                || drop(producer)
            });

            // return from hook
            state.messages.clone()
        },
        // initial state producer
        || UseSubscribeState { messages: Rc::new(RefCell::new(vec![])) },
    )
}"#
        ),
        p(vec![text(
            "\u{73b0}\u{5728}\u{6211}\u{4eec}\u{53ef}\u{4ee5}\u{4f7f}\u{7528}\u{81ea}\u{5b9a}\\
             u{4e49}\u{94a9}\u{5b50}\u{4e86}\u{ff1a}"
        )]),
        code_block(
            "rust",
            r#"#[function_component(ShowMessages)]
pub fn show_messages() -> Html {
    let state = use_subscribe();
    let output = state.borrow().deref().into_iter().map(|it| html! { <p>{ it }</p> });

    html! { <div>{ for output }</div> }
}"#
        ),
        p(vec![
            text(
                "\u{9700}\u{8981}\u{7279}\u{522b}\u{6ce8}\u{610f}\u{7684}\u{662f}\u{521b}\u{5efa}\\
                 \
                 u{81ea}\u{5b9a}\u{4e49}\u{94a9}\u{5b50}\u{65f6}"
            ),
            code("use_hook"),
            text(
                "\u{4e0d}\u{662f}\u{5fc5}\u{987b}\u{7684}\u{ff0c}\u{5b83}\u{4eec}\u{53ea}\u{662f}\\
                 \
                 u{7528}\u{6765}\u{5305}\u{542b}\u{5176}\u{4ed6}\u{94a9}\u{5b50}\u{3002}\u{901a}\\
                 u{5e38}\u{5e94}\u{907f}\u{514d}\u{4f7f}\u{7528}"
            ),
            code("use_hook"),
            text("\u{3002}"),
        ]),
        code_block(
            "rust",
            r#"fn use_subscribe() -> Rc<Vec<String>> {
    let (state, set_state) = use_state(Vec::new);

    use_effect(move || {
        let producer = EventBus::bridge(Callback::from(move |msg| {
            let mut messages = (*state).clone();
            messages.push(msg);
            set_state(messages)
        }));
        || drop(producer)
    });

    state
}"#
        ),
    ])
);
