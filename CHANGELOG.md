# Changelog

## ✨ **0.11** *(TBD)*

### Transition Guide

This release comes with a lot of breaking changes. We understand it's a hassle to update projects but the Yew team felt it was necessary to rip a few bandaids off now as we approach a 1.0 release in the (hopefully) near future. To ease the transition, here's a guide which outlines the main refactoring you will need to do for your project. (Note: all of the changes are reflected in the many example projects if you would like a proper reference example)

#### 1. Callback syntax

This is the main painful breaking change. It applies to both element listeners as well as `Component` callback properties. A good rule of thumb is that your components will now have to retain a `ComponentLink` to create callbacks on demand.

Before:
```rust
struct Model;

enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => true,
        }
    }

    fn view(&self) -> Html<Self> {
        // BEFORE: Callbacks were created implicitly from this closure syntax
        html! {
            <button onclick=|_| Msg::Click>{ "Click me!" }</button>
        }
    }
}
```

After:
```rust
struct Model {
  link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => true,
        }
    }

    fn view(&self) -> Html {
        // AFTER: Callbacks need to be explicitly created now
        let onclick = self.link.callback(|_| Msg::Click);
        html! {
            <button onclick=onclick>{ "Click me!" }</button>
        }
    }
}
```

If a closure has a parameter you will now need to specify the parameter's type.  A tip for finding the appropriate type is to search Yew's repo for the HTML attribute the closure is assigned to.

For example, `onkeydown` of `<button>`:

```
let onkeydown_callback = self.link.callback(|e: KeyDownEvent| {
    // ...
});
```

and

```
html! {
    <button
        onkeydown=onkeydown_callback,
        type="button">
        { "button" }
    </button>
}
```

#### 2. Method Renames

It should be safe to do a project-wide find/replace for the following:

- `send_self(` -> `send_message(`
- `send_back(` -> `callback(`
- `response(` -> `respond(`
- `AgentUpdate` -> `AgentLifecycleEvent`

These renames will probably require some more care:

- `fn handle(` -> `fn handle_input(` *(for Agent trait implementations)*

#### 3. Drop Generic Types for `Html<Self>` -> `Html`

:tada: We are pretty excited about this change! The generic type parameter
was confusing and restrictive and is now a thing of the past!

Before:
```rust
impl Component for Model {
    // ...

    fn view(&self) -> Html<Self> {
        html! { /* ... */ }
    }
}
```

After:
```rust
impl Component for Model {
    // ...

    fn view(&self) -> Html {
        html! { /* ... */ }
    }
}
```

- #### ⚡️ Features

  - The `html!` macro now accepts a `Callback` for element listeners. [[@jstarry], [#777](https://github.com/yewstack/yew/pull/777)]

  ```rust
  struct Model {
      onclick: Callback<ClickEvent>,
  }

  enum Msg {
      Click,
  }

  impl Component for Model {
      type Message = Msg;
      type Properties = ();

      fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
          Model {
              onclick: link.callback(|_| Msg::Click),
          }
      }

      fn update(&mut self, msg: Self::Message) -> ShouldRender {
          match msg {
              Msg::Click => true,
          }
      }

      fn view(&self) -> Html {
          html! {
              <button onclick=&self.onclick>{ "Click me!" }</button>
          }
      }
  }
  ```

  - Add `send_message_batch` method to `ComponentLink`. [[@hgzimmerman], [#748](https://github.com/yewstack/yew/pull/748)]
  - Allow compilation to `wasi` target without `wasm_bindgen`. [[@dunnock], [#746](https://github.com/yewstack/yew/pull/746)]
  - `ComponentLink` now implements `Clone` which enables `Future` usage without explicit Yew framework support. [[@hgzimmerman], [#749](https://github.com/yewstack/yew/pull/749)]

  ```rust
  use wasm_bindgen::JsValue;
  use wasm_bindgen_futures::future_to_promise;

  // future must implement `Future<Output = Component::Message> + 'static`
  let link = self.link.clone();
  let js_future = async move {
      link.send_message(future.await);
      Ok(JsValue::NULL)
  };

  future_to_promise(js_future);
  ```

- #### 🛠 Fixes

  - Creating a `Callback` with `ComponentLink` is no longer restricted to mutable references, improving ergonomics. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `Callback` `reform` method no longer consumes self making it easier to "reverse map" a `Callback`. [[@jstarry], [#779](https://github.com/yewstack/yew/pull/779)]

  ```rust
  pub struct ListHeader {
      props: Props,
  }

  #[derive(Properties)]
  pub struct Props {
      #[props(required)]
      pub on_hover: Callback<Hovered>,
      #[props(required)]
      pub text: String,
  }

  impl Component for ListHeader {
      type Message = ();
      type Properties = Props;

      fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
          ListHeader { props }
      }

      fn update(&mut self, _: Self::Message) -> ShouldRender {
          false
      }

      fn view(&self) -> Html {
          let onmouseover = self.props.on_hover.reform(|_| Hovered::Header);
          html! {
              <div class="list-header" onmouseover=onmouseover>
                  { &self.props.text }
              </div>
          }
      }
  }
  ```

  - Reduced allocations in the `Classes` `to_string` method. [[@hgzimmerman], [#772](https://github.com/yewstack/yew/pull/772)]
  - Empty string class names are now filtered out to prevent panics. [[@jstarry], [#770](https://github.com/yewstack/yew/pull/770)]

- #### 🚨 Breaking changes

  - Removed generic type parameter from `Html` and all virtual node types: `VNode`, `VComp`, `VTag`, `VList`, `VText`, etc. [[@jstarry], [#783](https://github.com/yewstack/yew/pull/783)]
  - Removed support for macro magic closure syntax for element listeners. (See above for how to pass a `Callback` explicitly instead). [[@jstarry], [#782](https://github.com/yewstack/yew/pull/782)]
  - Renamed `Agent` methods and event type for clarity. `handle` -> `handle_input`, `AgentUpdate` -> `AgentLifecycleEvent`, `response` -> `respond`. [[@philip-peterson], [#751](https://github.com/yewstack/yew/pull/751)]
  - The `ComponentLink` `send_back` method has been renamed to `callback` for clarity. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `ComponentLink` `send_self` and `send_self_batch` methods have been renamed to `send_message` and `send_message_batch` for clarity. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `Agent` `send_back` method has been renamed to `callback` for clarity. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `VTag` `children` value type has changed from `Vec<VNode>` to `VList`. [[@jstarry], [#754](https://github.com/yewstack/yew/pull/754)]


## ✨ **0.10** *(2019-11-11)*

- #### ⚡️ Features

  - `Future` support :tada: A `Component` can update following the completion of a `Future`. Check out [this example](https://github.com/yewstack/yew/tree/master/examples/futures) to see how it works. This approach was borrowed from a fork of Yew called [`plaster`](https://github.com/carlosdp/plaster) created by [@carlosdp]. [[@hgzimmerman], [#717](https://github.com/yewstack/yew/pull/717)]
  - Added the `agent` and `services` features so that this functionality can be disabled (useful if you are switching to using `Future`s). [[@hgzimmerman], [#684](https://github.com/yewstack/yew/pull/684)]
  - Add `ref` keyword for allowing a `Component` to have a direct reference to its rendered elements. For example, you can now easily focus an `<input>` element after mounting. [[@jstarry], [#715](https://github.com/yewstack/yew/pull/715)]

  ```rust
  use stdweb::web::html_element::InputElement;
  use stdweb::web::IHtmlElement;
  use yew::*;

  pub struct Input {
      node_ref: NodeRef,
  }

  impl Component for Input {
      type Message = ();
      type Properties = ();

      fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
          Input {
              node_ref: NodeRef::default(),
          }
      }

      fn mounted(&mut self) -> ShouldRender {
          if let Some(input) = self.node_ref.try_into::<InputElement>() {
              input.focus();
          }
          false
      }

      fn update(&mut self, _: Self::Message) -> ShouldRender {
          false
      }

      fn view(&self) -> Html<Self> {
          html! {
              <input ref=self.node_ref.clone() type="text" />
          }
      }
  }
  ```

  - Make `Agent` related types `public` to allow other crates to create custom agents. [[@dunnock], [#721](https://github.com/yewstack/yew/pull/721)]
  - `Component::change` will now return `false` for components that have `Component::Properties == ()`. [[@kellytk], [#690](https://github.com/yewstack/yew/pull/690)]]
  - Updated `wasm-bindgen` dependency to `0.2.54`. Please update your `wasm-bindgen-cli` tool by running `cargo install --force --version 0.2.54 -- wasm-bindgen-cli`. [[@jstarry], [#730](https://github.com/yewstack/yew/pull/730)], [[@ctaggart], [#681](https://github.com/yewstack/yew/pull/681)]

- #### 🛠 Fixes

  - Fixed the mount order of components. The root component will be mounted after all descendants have been mounted. [[@jstarry], [#725](https://github.com/yewstack/yew/pull/725)]
  - All public items now implement `Debug`. [[@hgzimmerman], [#673](https://github.com/yewstack/yew/pull/673)]

- #### 🚨 Breaking changes

  - Minimum rustc version has been bumped to `1.39.0` for `Future` support. [[@jstarry], [#730](https://github.com/yewstack/yew/pull/730)]
  - `Component` now has a required `view` method and automatically implements the `Renderable` trait. The `view` method in the `Renderable` trait has been renamed to `render`. [[@jstarry], [#563](https://github.com/yewstack/yew/pull/563)]

    Before:
    ```rust
    impl Component for Model {
        type Message = Msg;
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            Model {}
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            true
        }
    }

    impl Renderable<Model> for Model {
        fn view(&self) -> Html<Self> {
            html! { "hello" }
        }
    }
    ```

    After:
    ```rust
    impl Component for Model {
        type Message = Msg;
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            Model {}
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            true
        }

        fn view(&self) -> Html<Self> {
            html! { "hello" }
        }
    }
    ```

  - Removed the `Transferable` trait since it did no more than extend the serde `Serialize` and `Deserialize` traits. [[@hgzimmerman], [#319](https://github.com/yewstack/yew/pull/319)]

    Before:
    ```rust
    impl Transferable for Input {}
    #[derive(Serialize, Deserialize)]
    pub enum Input {
      Connect,
    }
    ```

    After:
    ```rust
    #[derive(Serialize, Deserialize)]
    pub enum Input {
      Connect,
    }
    ```
  - `WebSocketService::connect` will now return a `Result` in order to stop panicking on malformed urls. [[@lizhaoxian], [#727](https://github.com/yewstack/yew/pull/727)]
  - `VTag` now is boxed within `VNode` to shrink the size of its enum representation. [[@hgzimmerman], [#675](https://github.com/yewstack/yew/pull/675)]

## ✨ **0.9.2** *(2019-10-12)*

- #### 🛠 Fixes

  - Fix `yew-macro` dependency version

## ✨ **0.9.1** *(2019-10-12)*

Happy Canadian Thanksgiving! 🦃

- #### ⚡️ Features

  - Implemented `Default` trait for `VNode` so that `unwrap_or_default` can be called on `Option<Html<Self>>`. [[@hgzimmerman], [#672](https://github.com/yewstack/yew/pull/672)]
  - Implemented `PartialEq` trait for `Classes` so that is more ergonomic to use `Classes` type in component props. [[@hgzimmerman], [#680](https://github.com/yewstack/yew/pull/680)]
  - Updated `wasm-bindgen` dependency to `0.2.50`. Please update your `wasm-bindgen-cli` tool by running `cargo install --force --version 0.2.50 -- wasm-bindgen-cli`. [[@jstarry], [#695](https://github.com/yewstack/yew/pull/695)]

- #### 🛠 Fixes

  - Fixed issue where text nodes were sometimes rendered out of order. [[@jstarry], [#697](https://github.com/yewstack/yew/pull/697)]
  - Fixed regression introduced in 0.9.0 that prevented tag attributes from updating properly. [[@jstarry], [#698](https://github.com/yewstack/yew/pull/698)]
  - Fixed emscripten builds by pinning the version for the `ryu` downstream dependency. [[@jstarry], [#703](https://github.com/yewstack/yew/pull/703)]
  - Updated `stdweb` to `0.4.20` which fixed emscripten builds and unblocked updating `wasm-bindgen` to `0.2.50`. [[@ctaggart], [@jstarry], [#683](https://github.com/yewstack/yew/pull/683), [#694](https://github.com/yewstack/yew/pull/694)]
  - Cleaned up build warnings for missing `dyn` keywords. [[@benreyn], [#687](https://github.com/yewstack/yew/pull/687)]

## ✨ **0.9** *(2019-09-27)*

- #### ⚡️ Features

  - New `KeyboardService` for setting up key listeners on browsers which support the feature. [[@hgzimmerman], [#647](https://github.com/yewstack/yew/pull/647)]
  - `ComponentLink` can now create a `Callback` with more than one `Message`. The `Message`'s will be batched together so that the `Component` will not be re-rendered more than necessary. [[@stkevintan], [#660](https://github.com/yewstack/yew/pull/660)]
  - `Message`'s to `Public` `Agent`'s will now be queued if the `Agent` hasn't finished setting up yet. [[@serzhiio], [#596](https://github.com/yewstack/yew/pull/596)]
  - `Agent`'s can now be connected to without a `Callback`. Instead of creating a bridge to the agent, create a dispatcher like so: `MyAgent::dispatcher()`. [[@hgzimmerman], [#639](https://github.com/yewstack/yew/pull/639)]
  - `Component`'s can now accept children in the `html!` macro. [[@jstarry], [#589](https://github.com/yewstack/yew/pull/589)]

    ```rust
    // app.rs

    html! {
      <MyList name="Grocery List">
        <MyListItem text="Apples" />
      </MyList>
    }
    ```

    ```rust
    // my_list.rs

    use yew::prelude::*;

    pub struct MyList(Props);

    #[derive(Properties)]
    pub struct Props {
        #[props(required)]
        pub name: String,
        pub children: Children<MyListItem>,
    }

    impl Renderable<MyList> for MyList {
      fn view(&self) -> Html<Self> {
        html! {{
          self.props.children.iter().collect::<Html<Self>>()
        }}
      }
    }
    ```

  - `Iterator`s can now be rendered in the `html!` macro without using the `for` keyword. [[@hgzimmerman], [#622](https://github.com/yewstack/yew/pull/622)]

    Before:
    ```rust
    html! {{
      for self.props.items.iter().map(renderItem)
    }}
    ```

    After:
    ```rust
    html! {{
      self.props.items.iter().map(renderItem).collect::<Html<Self>>()
    }}
    ```

  - Closures are now able to be transformed into optional `Callback` properties. [[@Wodann], [#612](https://github.com/yewstack/yew/pull/612)]
  - Improved CSS class ergonomics with new `Classes` type. [[@DenisKolodin], [#585](https://github.com/yewstack/yew/pull/585)], [[@hgzimmerman], [#626](https://github.com/yewstack/yew/pull/626)]
  - Touch events are now supported `<div ontouchstart=|_| Msg::TouchStart>` [[@boydjohnson], [#584](https://github.com/yewstack/yew/pull/584)], [[@jstarry], [#656](https://github.com/yewstack/yew/pull/656)]
  - The `Component` trait now has an `mounted` method which can be implemented to react to when your components have been mounted to the DOM. [[@hgzimmerman], [#583](https://github.com/yewstack/yew/pull/583)]
  - Additional Fetch options `mode`, `cache`, and `redirect` are now supported [[@davidkna], [#579](https://github.com/yewstack/yew/pull/579)]
  - The derive props macro now supports Properties with lifetimes [[@jstarry], [#580](https://github.com/yewstack/yew/pull/580)]
  - New `ResizeService` for registering for `window` size updates [[@hgzimmerman], [#577](https://github.com/yewstack/yew/pull/577)]

- #### 🛠 Fixes

  - Fixed JS typo in RenderService. This was causing animation frames to not be dropped correctly. [[@jstarry], [#658](https://github.com/yewstack/yew/pull/658)]
  - Fixed `VNode` orphaning bug when destroying `VTag` elements. This caused some `Component`s to not be properly destroyed when they should have been. [[@hgzimmerman], [#651](https://github.com/yewstack/yew/pull/651)]
  - Fix mishandling of Properties `where` clause in derive_props macro [[@astraw], [#640](https://github.com/yewstack/yew/pull/640)]

- #### 🚨 Breaking changes

  None

## ✨ **0.8** *(2019-08-10)*

***Props! Props! Props!***

This release introduces a more developer friendly way to handle your `Component` props. Use the new `#[derive(Properties)]` macro to beef up your props! Property values can now be annotated as `#[props(required)]` which will enforce that props are present at compile time. This means that your props struct no longer needs to implement `Default`, so time to clean up all of those prop values you wrapped in `Option` to have a default value.

- #### ⚡️ Features

  - `html!` - Self-closing html tags can now be used: `<div class="marker" />` [[@totorigolo], [#523](https://github.com/yewstack/yew/pull/523)]
  - `html!` - SVG name-spaced tags are now supported! [[@jstarry], [#550](https://github.com/yewstack/yew/pull/550)]
  - Properties can now be required at compile time [[@jstarry], [#553](https://github.com/yewstack/yew/pull/525)]
  - App components can now be mounted with properties [[@jstarry], [#567](https://github.com/yewstack/yew/pull/567)]
  - Apps can now be mounted as the `<body>` tag [[@jstarry], [@kellytk], [#540](https://github.com/yewstack/yew/pull/540)]
  - Content editable elements can now trigger `oninput` events [[@tiziano88], [#549](https://github.com/yewstack/yew/pull/549)]

- #### 🛠 Fixes

  - `html!` - Class name order is now preserved which unlocks the use of Semantic UI [[@charvp], [#424](https://github.com/yewstack/yew/pull/424)]
  - `html!` - Dashed tag names and properties are supported [[@jstarry], [#512](https://github.com/yewstack/yew/pull/512), [#550](https://github.com/yewstack/yew/pull/550)]
  - `html!` - All rust keywords can be used as tag attributes [[@jstarry], [#550](https://github.com/yewstack/yew/pull/550)]
  - `html!` - Support `Callback` closure with explicit return type [[@totorigolo], [#564](https://github.com/yewstack/yew/pull/564)]
  - `html!` - Fixed edge case where `>` token would break parser [[@totorigolo], [#565](https://github.com/yewstack/yew/pull/565)]
  - Performance improvement to the diff engine [[@totorigolo], [#539](https://github.com/yewstack/yew/pull/539)]
  - `Properties` no longer need to implement the `PartialEq`, `Clone`, or `Default` traits [[@jstarry], [#553](https://github.com/yewstack/yew/pull/553)]
  - `Component` will not panic if the `change` method is unimplemented [[@jstarry], [#554](https://github.com/yewstack/yew/pull/554)]

- #### 🚨 Breaking changes

  - The `Component::Properties` associated type must implement the new `Properties` trait [[@jstarry], [#553](https://github.com/yewstack/yew/pull/553)]

    The new `Properties` trait is what powers the ability to check required props are present at compile time. Use the derive props macro to implement automatically.

    ```rust
    use yew::Properties;

    #[derive(Properties)]
    pub struct Props {
      #[props(required)]
      pub value: MyStruct,
    }
    ```

  - `Callback` props no longer transform into `Option` types [[@jstarry], [#553](https://github.com/yewstack/yew/pull/553)]

    ```rust
    html! { <Button on_click=Msg::Click /> }
    ```

    ***before:***

    ```rust
    #[derive(PartialEq, Clone, Default)]
    pub struct Props {
        on_click: Option<Callback<()>>,
    }
    ```

    ***after:*** *note the `#[props(required)]` attribute*

    ```rust
    #[derive(PartialEq, Properties)]
    pub struct Props {
        #[props(required)]
        on_click: Callback<()>,
    }
    ```

## ✨ **0.7** *(2019-07-19)*

***Commas? We don't need no stinkin' commas!***

This release brings a new and improved `html!` macro for writing JSX-like syntax. Commas and colons are no longer necessary now that the macro is written as a procedural macro.

- #### ⚡️ Features
  - `html!{}` is now valid syntax and can be used to render nothing [[@jstarry], [#500](https://github.com/yewstack/yew/pull/500)]
  - Apps can now be built without `cargo-web` using `wasm-bindgen` [[@jstarry], [#497](https://github.com/yewstack/yew/pull/497)]
  - `Callback` now implements `Debug` [[@DenisKolodin], [#485](https://github.com/yewstack/yew/pull/485)]
  - New utility method for getting the `host` of the current page [[@DenisKolodin], [#509](https://github.com/yewstack/yew/pull/509)]

- #### 🛠 Fixes
  - `html!` - Commas are no longer necessary for splitting up attributes [[@jstarry], [#500](https://github.com/yewstack/yew/pull/500)]
  - `html!` - Colons are no longer necessary for denoting a `Component` tag [[@jstarry], [#500](https://github.com/yewstack/yew/pull/500)]
  - Textarea value can be now be set: `<textarea value="content">` [[@DenisKolodin], [#476](https://github.com/yewstack/yew/pull/476)]
  -  changed `StorageService::restore` to take an immutable receiver [[@dermetfan], [#480](https://github.com/yewstack/yew/pull/480)]
  - Fixed a component rendering bug [[@jstarry], [#502](https://github.com/yewstack/yew/pull/502)]

## ✨ **0.6** *(2019-02-20)*

- #### ⚡️ Features
  - Added `start_app` convenience method for initializing the app and mounting it to the body [[@DenisKolodin], [#462](https://github.com/yewstack/yew/pull/462)]
  - Added handling of files of `input` element. There is now a `ChangeData::Files` variant for the `onchange` handler [[@DenisKolodin], [#464](https://github.com/yewstack/yew/pull/464)]
  - Added `ReaderService` to read data from `File` instances. [[@DenisKolodin], [#464](https://github.com/yewstack/yew/pull/464), [#468](https://github.com/yewstack/yew/pull/468)]

- #### 🛠 Fixes
  - It was impossible to set `value` attribute for any tag instead of `option`, because it used
  inner value of `VTag` to keep the value for `input` element. Now `value` attribute works
  for `options`, `progress` tags, etc.


- #### 🔮 Examples
  - New example `file_upload` that prints sizes of uploaded files [[@DenisKolodin], [#464](https://github.com/yewstack/yew/pull/464)]

## ✨ **0.5** *(2019-02-01)*

**🎶 Secret Agent Man 🎶**

This release introduces the concept of an `Agent`. Agents are separate activities which you could run in the same thread or in a separate thread. There are three types of agents `Context`, `Job`, `Public` described below. To connect to an agent use the `Worker::bridge` method and pass a link of component's environment to it.

- #### ⚡️ Features
  - Introduced the concept of an `Agent` which can run processes in other contexts:
    - `Context` agent spawns once per thread
    - `Job` agent spawns for every bridge
    - `Public` agent spawns an agent in a separate thread (it uses [Web Workers API] under the hood).
  - Allow setting the whole properties struct of a component with `<Component: with props />`
  - `ComponentLink` now has a `send_self` method which allows components to update themselves [[@DenisKolodin], [#365](https://github.com/yewstack/yew/pull/365)]
  - All services are re-exported within the `yew::services` module.
  - `html!` macro supports multiple classes in a single string:
  `<a class="button is-primary",>`.
  - Added `FetchOptions` to allow setting `Credentials` of `fetch` request.
  - `FetchService` aborts requests using `AbortController`.
  - Added `SubmitEvent` with `onsubmit` rule.


- #### 🛠 Fixes

  - Bug with emscripten target `RuntimeError: index out of bounds` fixed with a new scheduler [[@DenisKolodin], [#272](https://github.com/yewstack/yew/pull/272)]

- #### 🚨 Breaking changes
  - `send_back` method requires a mutable reference to `self`. This was added to prevent creating callbacks in `view` implementations. [[@DenisKolodin], [#367](https://github.com/yewstack/yew/pull/367)]
  - `Context` requirement removed. It's no longer necessary to use `Component<CTX>` type parameter. Instead, a link to the environment is provided with the `Component::create` call. [[@DenisKolodin], [#272](https://github.com/yewstack/yew/pull/272)]

## ✨ **0.4** *(2018-06-01)*
## ✨ **0.3** *(2018-04-23)*
## ✨ **0.2** *(2018-01-08)*
## ✨ **0.1** *(2017-12-31)*

[Web Workers API]: https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API
[@astraw]: https://github.com/astraw
[@boydjohnson]: https://github.com/boydjohnson
[@carlosdp]: https://github.com/carlosdp
[@charvp]: https://github.com/charvp
[@ctaggart]: https://github.com/ctaggart
[@davidkna]: https://github.com/davidkna
[@DenisKolodin]: https://github.com/DenisKolodin
[@dermetfan]: https://github.com/dermetfan
[@dunnock]: https://github.com/dunnock
[@hgzimmerman]: https://github.com/hgzimmerman
[@jstarry]: https://github.com/jstarry
[@kellytk]: https://github.com/kellytk
[@lizhaoxian]: https://github.com/lizhaoxian
[@philip-peterson]: https://github.com/philip-peterson
[@serzhiio]: https://github.com/serzhiio
[@stkevintan]: https://github.com/stkevintan
[@tiziano88]: https://github.com/tiziano88
[@totorigolo]: https://github.com/totorigolo
[@Wodann]: https://github.com/Wodann
