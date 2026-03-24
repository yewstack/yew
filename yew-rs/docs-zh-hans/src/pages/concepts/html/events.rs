pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("介绍")],
        p![
            text("Yew 与 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/",
                code("web-sys"),
            ),
            text(" crate 集成，并使用该 crate 中的事件。下面的"),
            link!("#event-types", text("表格")),
            text("列出了在 "),
            code("html!"),
            text(" 宏中接受的所有 "),
            code("web-sys"),
            text(" 事件。"),
        ],
        p![
            text("您仍然可以为下表中未列出的事件添加 "),
            link!("/zh-Hans/docs/concepts/function-components/callbacks", code("Callback")),
            text("，请参见"),
            link!("#manual-event-listener", text("手动事件监听器")),
            text("。"),
        ],
        h2![text("事件类型")],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                text("所有的事件类型都在 "),
                code("yew::events"),
                text(" 下重新导出。\n使用 "),
                code("yew::events"),
                text(" 中的类型比手动将 "),
                code("web-sys"),
                text(
                    " 作为依赖项包含在您的 crate 中更容易确保版本兼容性，\n因为您不会使用与 Yew 指定的版本冲突的版本。",
                ),
            ],
        ],
        p![
            text("事件监听器的名称是在 "),
            code("html"),
            text(" 宏中添加事件 "),
            code("Callback"),
            text(" 时预期的名称："),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <button onclick={Callback::from(|_| ())}>
    //      ^^^^^^^ event listener name
        { "Click me!" }
    </button>
};"#,
        ),
        p![
            text("事件名称是监听器名称去掉 \"on\" 前缀，因此 "),
            code("onclick"),
            text(" 事件监听器监听 "),
            code("click"),
            text(" 事件。查看本页末尾的"),
            link!("#available-events", text("完整事件列表")),
            text("及其类型。"),
        ],
        h2_id!("event-bubbling", text("事件捕获")),
        p![
            text("Yew 调度的事件遵循虚拟 DOM 层次结构，向上冒泡到监听器。目前，仅支持监听器的冒泡阶段。请注意，虚拟 DOM 层次结构通常（但并非总是）与实际 DOM 层次结构相同。在处理"),
            link!("/zh-Hans/docs/advanced-topics/portals", text("传送门")),
            text("和其他更高级技术时，这一区别很重要。对于良好实现的组件，直觉应该是事件从子组件冒泡到父组件。这样，您在 "),
            code("html!"),
            text(" 中编写的层次结构就是事件处理程序观察到的层次结构。"),
        ],
        p![text(
            "如果您不想要事件冒泡，可以通过调用",
        )],
        code_block(
            "rust",
            r#"yew::set_event_bubbling(false);"#,
        ),
        p![
            text("在启动应用程序"),
            italic![text("之前")],
            text("。这会加快事件处理速度，但某些组件可能会因未收到预期的事件而中断。请谨慎使用！"),
        ],
        h2![text("事件委托")],
        p![text(
            "可能会让人惊讶的是，事件监听器并不是直接注册在它们被渲染的元素上。相反，事件是从 Yew 应用的子树根节点委托的。不过，事件仍然以其原生形式传递，并且不会创建任何合成形式。这可能会导致 HTML 监听器中预期的事件与 Yew 中出现的事件之间的不匹配。",
        )],
        ul![
            li![
                link!(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.current_target",
                    code("Event::current_target"),
                ),
                text(" 指向 Yew 子树根节点，而不是添加监听器的元素。如果您想访问底层的 "),
                code("HtmlElement"),
                text("，请使用 "),
                link!("/zh-Hans/docs/concepts/function-components/node-refs", code("NodeRef")),
                text("。"),
            ],
            li![
                link!(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.event_phase",
                    code("Event::event_phase"),
                ),
                text(" 始终是 "),
                link!(
                    "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#associatedconstant.CAPTURING_PHASE",
                    code("Event::CAPTURING_PHASE"),
                ),
                text("。在内部，事件将表现得像是在冒泡阶段，事件传播将被重放，并且事件会"),
                link!("#event-bubbling", text("向上冒泡")),
                text("，即虚拟 DOM 中较高的事件监听器将在较低的事件监听器之后触发。目前，Yew 不支持捕获监听器。"),
            ],
        ],
        p![text(
            "这也意味着由 Yew 注册的事件通常会在其他事件监听器之前触发。",
        )],
        h2![text("具备类型的事件目标")],
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                text("在本节中，"),
                bold![
                    text("target ("),
                    link!(
                        "https://developer.mozilla.org/en-US/docs/Web/API/Event/target",
                        code("Event.target"),
                    ),
                    text(")"),
                ],
                text(" 始终指的是事件从其派发的元素。"),
            ],
            p![
                text("这"),
                bold![text("不一定")],
                text("总是指代 "),
                code("Callback"),
                text(" 所放置的元素。"),
            ],
        ],
        p![
            text("在事件 "),
            code("Callback"),
            text(" 中，您可能希望获取该事件的目标。例如，"),
            code("change"),
            text(" 事件没有提供任何信息，但用于通知某些内容已更改。"),
        ],
        p![
            text("在 Yew 中，以正确的类型获取目标元素可以通过几种方式完成，我们将在这里逐一介绍。调用事件上的 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html#method.target",
                code("web_sys::Event::target"),
            ),
            text(" 返回一个可选的 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.EventTarget.html",
                code("web_sys::EventTarget"),
            ),
            text(" 类型，当您想知道输入元素的值时，这可能看起来不是很有用。"),
        ],
        p![text(
            "在下面的所有方法中，我们将解决相同的问题，以便清楚地了解方法的不同之处，而不是手头的问题。",
        )],
        p![bold![text("问题：")]],
        p![
            text("我们在 "),
            code("<input>"),
            text(" 元素上有一个 "),
            code("onchange"),
            text(" "),
            code("Callback"),
            text("，每次调用时，我们希望向组件发送一个"),
            link!("/zh-Hans/docs/concepts/html/components#update", text("更新")),
            text(" "),
            code("Msg"),
            text("。"),
        ],
        p![
            text("我们的 "),
            code("Msg"),
            text(" 枚举如下所示："),
        ],
        code_block(
            "rust",
            r#"pub enum Msg {
    InputValue(String),
}"#,
        ),
        h3![text("使用 "), code("JsCast")],
        p![
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                code("wasm-bindgen"),
            ),
            text(" crate 有一个有用的 trait："),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                code("JsCast"),
            ),
            text("，它允许我们在类型之间直接转换，只要它实现了 "),
            code("JsCast"),
            text(" 就行。我们可以谨慎地转换，这涉及运行时检查和处理 "),
            code("Option"),
            text(" 和 "),
            code("Result"),
            text(" 的逻辑，或者我们也可以冒险直接强行转换。"),
        ],
        p![text("多说无益，看代码：")],
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[dependencies]
# 需要 wasm-bindgen 用于调用 JsCast
wasm-bindgen = "0.2""#,
        ),
        code_block(
            "rust",
            r#"//highlight-next-line
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            // 当事件被创建时，目标是未定义的，只有在派发时才会添加目标。
            let target: Option<EventTarget> = e.target();
            // 事件可能会冒泡，因此此侦听器可能会捕获不是 HtmlInputElement 类型的子元素的事件。
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
        // 你必须了解 target 是 HtmlInputElement，否则调用 value 将是未定义行为（UB）。
        // 在这里，我们确信这是输入元素，因此我们可以在不检查的情况下将其转换为适当的类型。
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
}"#,
        ),
        p![
            code("JsCast"),
            text(" 提供的方法是 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                code("dyn_into"),
            ),
            text(" 和 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.unchecked_into",
                code("unchecked_into"),
            ),
            text("。 如你所见，它们允许我们从 "),
            code("EventTarget"),
            text(" 转换为 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.HtmlInputElement.html",
                code("HtmlInputElement"),
            ),
            text("。 "),
            code("dyn_into"),
            text(" 方法是谨慎的，因为它会在运行时检查类型是否实际为 "),
            code("HtmlInputElement"),
            text("，如果不是则返回 "),
            code("Err(JsValue)"),
            text("。"),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html",
                code("JsValue"),
            ),
            text(" 是一个通用类型，将原来的对象返回给你，以便再次尝试转换为别的类型。"),
        ],
        p![
            text("这会儿你可能会想，什么时候可以使用危险版本？在上面的情况下，它是安全的"),
            sup![text("1")],
            text("，因为我们将 "),
            code("Callback"),
            text(" 设置在一个没有子元素的元素上，所以目标只能是同一个元素。"),
        ],
        p![
            italic![
                sup![text("1")],
                text(" 只要涉及到 JS 领域，就是安全的。"),
            ],
        ],
        h3![text("使用 "), code("TargetCast")],
        p![bold![
            text("强烈建议先阅读 "),
            link!("#using-jscast", text("使用 JsCast")),
            text("！"),
        ]],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("TargetCast"),
                text(" 的设计目的是让新用户了解 "),
                code("JsCast"),
                text(" 的行为，但范围更小，仅涉及事件及其目标。"),
            ],
            p![
                text("选用 "),
                code("TargetCast"),
                text(" 或 "),
                code("JsCast"),
                text(" 纯粹是个人偏好，实际您会发现 "),
                code("TargetCast"),
                text(" 的实现和 "),
                code("JsCast"),
                text(" 的功能很相似。"),
            ],
        ],
        p![
            code("TargetCast"),
            text(" trait 是在 "),
            code("JsCast"),
            text(" 基础之上构建的，专门用于从事件中获取类型化的事件目标。"),
        ],
        p![
            code("TargetCast"),
            text(" 是 Yew 的一部分，因此无需添加依赖项即可在事件上使用 trait 方法，但它的工作方式与 "),
            code("JsCast"),
            text(" 非常相似。"),
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[component]
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
        // 你必须清楚 target 是 HtmlInputElement，否则调用 value 将是未定义行为（UB）。
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
}"#,
        ),
        p![
            text("如果您已经了解了 "),
            code("JsCast"),
            text("，或者了解了这个 trait，您可能会发现 "),
            code("TargetCast::target_dyn_into"),
            text(" 与 "),
            code("JsCast::dyn_into"),
            text(" 相似，但专门用于事件的目标。"),
            code("TargetCast::target_unchecked_into"),
            text(" 与 "),
            code("JsCast::unchecked_into"),
            text(" 类似，因此上面关于 "),
            code("JsCast"),
            text(" 的所有警告都适用于 "),
            code("TargetCast"),
            text("。"),
        ],
        h3![text("使用 "), code("NodeRef")],
        p![
            link!("/zh-Hans/docs/concepts/function-components/node-refs", code("NodeRef")),
            text(" 可以代替查询给定给 "),
            code("Callback"),
            text(" 的事件。"),
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[component]
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
}"#,
        ),
        p![
            text("通过 "),
            code("NodeRef"),
            text("，你可以忽略事件并使用 "),
            code("NodeRef::cast"),
            text(" 方法获取一个 "),
            code("Option<HtmlInputElement>"),
            text(" - 这是可选的，因为在设置 "),
            code("NodeRef"),
            text(" 之前调用 "),
            code("cast"),
            text("，或者类型不匹配时将返回 "),
            code("None"),
            text("。"),
        ],
        p![
            text("你可能会看到，通过使用 "),
            code("NodeRef"),
            text("，我们不必将 "),
            code("String"),
            text(" 发送回状态，因为我们总是访问 "),
            code("input_node_ref"),
            text(" - 因此我们可以这样做："),
        ],
        code_block(
            "rust",
            r#"use web_sys::HtmlInputElement;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let input_node_ref = use_node_ref();

    //highlight-start
    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                // 对 value 做点什么
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
}"#,
        ),
        p![
            text("您选择哪种方法取决于您的组件和您的偏好，没有所谓的"),
            italic![text("推荐")],
            text("方法。"),
        ],
        h2![text("手动事件监听器")],
        p![
            text("您可能希望监听 Yew 的 "),
            code("html"),
            text(" 宏不支持的事件，查看"),
            link!("#event-types", text("这里列出的支持的事件")),
            text("。"),
        ],
        p![
            text("为了手动向某个元素添加事件监听器，我们需要借助 "),
            link!("/zh-Hans/docs/concepts/function-components/node-refs", code("NodeRef")),
            text("，以便在 "),
            code("use_effect_with"),
            text(" 中使用 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/index.html",
                code("web-sys"),
            ),
            text(" 和 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/index.html",
                code("wasm-bindgen"),
            ),
            text(" API 添加监听器。"),
        ],
        p![
            text("以下示例将展示如何为虚构的 "),
            code("custard"),
            text(" 事件添加监听器。所有不受 yew 支持的事件或自定义事件都可以表示为 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Event.html",
                code("web_sys::Event"),
            ),
            text("。如果您需要访问自定义/不受支持事件的特定方法或字段，可以使用 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html",
                code("JsCast"),
            ),
            text(" 的方法将其转换为所需的类型。"),
        ],
        h3![text("使用 "), code("Closure"), text("（冗长版本）")],
        p![
            text("直接使用 "),
            code("web-sys"),
            text(" 和 "),
            code("wasm-bindgen"),
            text(" 的接口可能有点痛苦……所以要有点心理准备（"),
            link!(
                "#using-gloo-concise",
                text("感谢 "), code("gloo"), text("，有了更简洁的方法"),
            ),
            text("）。"),
        ],
        code_block(
            "rust",
            r#"use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let div_node_ref = use_node_ref();

    use_effect_with(
        div_node_ref.clone(),
        {
            let div_node_ref = div_node_ref.clone();

            move |_| {
                let mut custard_listener = None;

                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    // 创建您通常会创建的 Callback
                    let oncustard = Callback::from(move |_: Event| {
                        // 对 custard 做点什么..
                    });

                    // 从 Box<dyn Fn> 创建一个 Closure - 这必须是 'static
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
        }
    );

    html! {
        <div ref={div_node_ref} id="my-div"></div>
    }
}"#,
        ),
        p![
            text("有关 "),
            code("Closure"),
            text(" 的更多信息，请参见 "),
            link!(
                "https://wasm-bindgen.github.io/wasm-bindgen/examples/closures.html",
                text("wasm-bindgen 指南"),
            ),
            text("。"),
        ],
        h3![text("使用 "), code("gloo"), text("（简洁版本）")],
        p![
            text("更方便的方法是使用 "),
            code("gloo"),
            text("，更具体地说是 "),
            link!(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/index.html",
                code("gloo_events"),
            ),
            text("， 它是 "),
            code("web-sys"),
            text("、"),
            code("wasm-bindgen"),
            text(" 的高层抽象实现。"),
        ],
        p![
            code("gloo_events"),
            text(" 提供了 "),
            code("EventListener"),
            text(" 类型，可以用于创建和存储事件监听器。"),
        ],
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[dependencies]
gloo-events = "0.1""#,
        ),
        code_block(
            "rust",
            r#"use web_sys::HtmlElement;
use yew::prelude::*;

use gloo::events::EventListener;

#[component]
fn MyComponent() -> Html {
    let div_node_ref = use_node_ref();

    use_effect_with(
        div_node_ref.clone(),
        {
            let div_node_ref = div_node_ref.clone();

            move |_| {
                let mut custard_listener = None;

                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    // 创建您通常会创建的 Callback
                    let oncustard = Callback::from(move |_: Event| {
                        // 对 custard 做点什么..
                    });

                    // 从 Box<dyn Fn> 创建一个 Closure - 这必须是 'static
                    let listener = EventListener::new(
                        &element,
                        "custard",
                        move |e| oncustard.emit(e.clone())
                    );

                    custard_listener = Some(listener);
                }

                move || drop(custard_listener)
            }
        }
    );

    html! {
        <div ref={div_node_ref} id="my-div"></div>
    }
}"#,
        ),
        p![
            text("有关 "),
            code("EventListener"),
            text(" 的更多信息，请参见 "),
            link!(
                "https://docs.rs/gloo-events/0.1.1/gloo_events/struct.EventListener.html",
                text("gloo_events docs.rs"),
            ),
            text("。"),
        ],
        h2_id!("available-events", text("可用事件的完整列表")),
        table(
            vec![vec![text("侦听器名称")], vec![code("web_sys"), text(" 事件类型")]],
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
}

crate::doc_page!("事件", "/zh-Hans/docs/concepts/html/events", page_content());
