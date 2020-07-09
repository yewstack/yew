---
description: Both HTML and SVG elements are supported
---

# Elements

## Tag Structure

Element tags must either self-close `<... />` or have a corresponding close tag for each open tag

{% tabs %}
{% tab title="Open - Close" %}
```rust
html! {
  <div id="my_div"></div>
}
```
{% endtab %}

{% tab title="INVALID" %}
```rust
html! {
  <div id="my_div"> // <- MISSING CLOSE TAG
}
```
{% endtab %}

{% tab title="Self-Closing" %}
```rust
html! {
  <input id="my_input" />
}
```
{% endtab %}

{% tab title="INVALID" %}
```rust
html! {
  <input id="my_input"> // <- MISSING SELF-CLOSE
}
```
{% endtab %}
{% endtabs %}

{% hint style="info" %}
For convenience, elements which _usually_ require a closing tag are **allowed** to self-close. For example, writing `html! { <div class="placeholder" /> }` is valid.
{% endhint %}

## Children

Create complex nested HTML and SVG layouts with ease:

{% tabs %}
{% tab title="HTML" %}
```rust
html! {
    <div>
        <div data-key="abc"></div>
        <div class="parent">
            <span class="child" value="anything"></span>
            <label for="first-name">{ "First Name" }</label>
            <input type="text" id="first-name" value="placeholder" />
            <input type="checkbox" checked=true />
            <textarea value="write a story" />
            <select name="status">
                <option selected=true disabled=false value="">{ "Selected" }</option>
                <option selected=false disabled=true value="">{ "Unselected" }</option>
            </select>
        </div>
    </div>
}
```
{% endtab %}

{% tab title="SVG" %}
```rust
html! {
    <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
        <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
        <g filter="url(#filter0_d)">
            <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
            <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
        </g>
        <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
        <defs>
            <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                <feGaussianBlur stdDeviation="2"/>
                <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
            </filter>
        </defs>
    </svg>
}
```
{% endtab %}
{% endtabs %}

## Classes

There are a number of convenient options for specifying classes for an element:

{% tabs %}
{% tab title="Literal" %}
```rust
html! {
  <div class="container"></div>
}
```
{% endtab %}

{% tab title="Multiple" %}
```rust
html! {
  <div class="container center-align"></div>
}
```
{% endtab %}

{% tab title="Interpolated" %}
```rust
html! {
  <div class=format!("{}-container", size)></div>
}
```
{% endtab %}

{% tab title="Expression" %}
```rust
html! {
  <div class=self.classes()></div>
}
```
{% endtab %}

{% tab title="Tuple" %}
```rust
html! {
  <div class=("class-1", "class-2")></div>
}
```
{% endtab %}

{% tab title="Vector" %}
```rust
html! {
  <div class=vec!["class-1", "class-2"]></div>
}
```
{% endtab %}
{% endtabs %}

## Listeners

Listener attributes need to be passed a `Callback` which is a wrapper around a closure. How you create your callback depends on how you wish your app to react to a listener event:

{% tabs %}
{% tab title="Component Handler" %}
```rust
struct MyComponent {
    link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MyComponent { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                // Handle Click
            }
        }
    }

    fn view(&self) -> Html {
        // Create a callback from a component link to handle it in a component
        let click_callback = self.link.callback(|_: ClickEvent| Msg::Click);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```
{% endtab %}

{% tab title="Agent Handler" %}
```rust
struct MyComponent {
    worker: Dispatcher<MyWorker>,
}

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent {
            worker: MyWorker::dispatcher()
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Create a callback from a worker to handle it in another context
        let click_callback = self.worker.callback(|_: ClickEvent| WorkerMsg::Process);
        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```
{% endtab %}

{% tab title="Other Cases" %}
```rust
struct MyComponent;

impl Component for MyComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Create an ephemeral callback
        let click_callback = Callback::from(|| {
            ConsoleService::new().log("clicked!");
        });

        html! {
            <button onclick=click_callback>
                { "Click me!" }
            </button>
        }
    }
}
```
{% endtab %}
{% endtabs %}

