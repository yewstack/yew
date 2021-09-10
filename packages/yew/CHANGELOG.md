# Changelog

## ✨ Yew **0.19.0** *(TBD)*

- #### 🛠 Fixes
  - [Fix support for boolean attributes in the `html!` macro](https://github.com/yewstack/yew/pull/2051).
  - [Fix `VTag` reuse to reset ancestor `NodeRef`](https://github.com/yewstack/yew/pull/2030).
  - [Fix `onsubmit` handler to take `Event` and not `FocusEvent`](https://github.com/yewstack/yew/pull/2009).
  - [Fix Clippy warnings with the `Properties` derive macro](https://github.com/yewstack/yew/pull/2007).
  - [Fix scheduler main queue delay](https://github.com/yewstack/yew/pull/1954).
  - [Fix some Clippy warnings when using the `Properties` derive macro](https://github.com/yewstack/yew/pull/1929).
  - [Fix inability to set the `autoplay` attribute](https://github.com/yewstack/yew/pull/1866).

- #### ⚡️ Features
  - [Component lifecycle scheduler optimizations](https://github.com/yewstack/yew/pull/2065).
    Deduplicated some lifecycle events, reduced indirection and branching for lifecycle event handlers,
    reduced scheduler calls and memory foot print, and optimized task priorities and batching.
  - [Allow taking `Fn(EVENT)` as a `Callback`](https://github.com/yewstack/yew/pull/1989).
    
    This adds support in the `html!` macro to consider a `Fn(EVENT)` as a
    `Callback` - this is a nice ergonomic improvement for `Function Component`s: 

    ```rust
    #[function_component(App)]
    fn app() -> Html {
      use yew::web_sys::console;

      let onclick = |_| console::log_1(&"Hello, World!".into());

      html! {
        <button {onclick}>{ "Click me!" }</button>
      }
    }
    ```
  - [Add support for const generics when using the `Properties` derive macro](https://github.com/yewstack/yew/pull/1978).

    As of Rust 1.51 const generics are stabilized and now it can be used in `Properties`:
    ```rust
    #[derive(Clone, PartialEq, Properties)]
    pub struct FooProps<T, const N: usize>
    where
      T: Clone + PartialEq,
    {
      bar: [T; N],
    }
    ```

  - [Add shorthand syntax for `Properties` and attributes](https://github.com/yewstack/yew/pull/1970).

    This allows you to omit the property or attribute name if the variable itself has the same name: 

    ```rust
    let id = "mydiv";
    let onclick = Callback::from(|_| ());

    html! {
      <div {id} {onclick}>
      </div>
    }
    ```

    This works both on elements and components so enjoy saving yourself from typing another
    redundant `onclick={onclick}`!

  - [Static attribute lists](https://github.com/yewstack/yew/pull/1962). This reduces allocation
  sizes for dynamic attribute lists and when possible will crate arrays for static attribute lists
  so that comparisons can be done even faster.
  - [Optimize `VList` diffing and patching](https://github.com/yewstack/yew/pull/1555). This
  optimizes diffing and patching for both keyed and unkeyed lists for better performance with long
  lists.
  - [Optimize `VTag` construction, memory footprint and patching](https://github.com/yewstack/yew/pull/1947).
  This moves more `VTag` construction logic to be compile-time using the `html!` macro, reduces
  footprint by using enums, and reduces enum branching during `VTag` patching for another boost
  of performance.
  - [Add support for creating `Callback` from a `Future`](https://github.com/yewstack/yew/pull/1842).
  - [Add Function Components and Hooks](https://github.com/yewstack/yew/pull/1842).

    `Function Component` has been one of the top requested feature of Yew for awhile, the waiting
    can stop because it's finally here 🎉.

    ```rust
    #[function_component(App)]
    fn app() -> Html {
      html! {
        { "Hello, Yew 0.19.0!" }
      }
    }
    ```

    A `Function Component` is limited without a `Hook` or two so we threw them in too for good measure:

    ```rust
      #[function_component(Counter)]
      fn counter() -> Html {
        let counter = use_state(|| 0);

        let onclick = {
          let counter = counter.clone();
          Callback::from(move |_| counter.set(*counter + 1))
        };

        html! {
          <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
              <b>{ "Current value: " }</b>
              { *counter }
            <p>
          </div>
        }
      }
      ```
    
    Yew will release with the following "pre-defined" hooks: 
    - `use_state`
    - `use_ref`
    - `use_reducer`
    - `use_reducer_with_init`
    - `use_effect`
    - `use_effect_with_deps`
    - `use_context`

    Yew will also provide the ability to create custom hooks by exposing the `use_hook` function
    which is the common base that all the hooks are built from.

  - [Add `focusin` and `focusout` event support in the `html!` macro](https://github.com/yewstack/yew/pull/1945).
  - [`&str` can now be used for `Option<String>` and `Option<Cow<'static, str>>`](https://github.com/yewstack/yew/pull/1895).
  - [Reduce scheduler call indirection](https://github.com/yewstack/yew/pull/1903). This brings
  an 11% performance increase.
  - [Optimize `VTag` construction](https://github.com/yewstack/yew/pull/1867). This reduces some
  copying and reallocations when constructing `VTag` for a nice performance boost. 

- #### 🚨 Breaking changes
  - [Replace `with` expression for `..props` (props base expression)](https://github.com/yewstack/yew/pull/2024).

    This can be used in conjunction with the `props!` macro just like with `with` syntax:

    ```rust
    #[derive(PartialEq, Properties)]
    struct Props {
      #[prop_or(String::from("Elm"))]
      name: String,
      value: u32,
    }

    let props = props!{Props {
      value: 20,
    }};

    html! {
      <Comp name="Yew" ..props />
    }
    ```

    The restriction using `with` was that it was all or nothing; you couldn't mix using the `with`
    syntax and overriding property assignment, but this is not the case with the new props base
    expression!

    In the above example the `Comp` `name` property will have "Yew" as it's value, this works in a
    similar way as Rust's struct syntax also known as
    [Functional update syntax](https://doc.rust-lang.org/stable/reference/expressions/struct-expr.html#functional-update-syntax).

  - [Event listener multiplexer](https://github.com/yewstack/yew/pull/1542). This reduces overhead 
  of creating and dropping event listeners by using a global multiplexer for another performance 
  bump.

    All Event handlers created in the `html!` macro are attached to the `body` element to reduce
    tree traversal on propagation - The event target will still be the same, currentTarget will
    always be the `body` element.
  
    This also adds the ability to create a
    [passive](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)
    `Callback`:

    ```rust
    // ..
    fn view(&self, ctx: &Context<Self>) -> Html {
      // passive because we aren't calling `prevent_default` on the event!
      let onclick = ctx.link().callback_with_passive(true, |_| Msg::Clicked);
    }
    ```
    A passive event listeners can improve performance but only when the handler does not call
    `prevent_default` on the event.

    Adds a global control to disable event bubbling which if not required brings a big performance
    increase.

  - [Components trait V2](https://github.com/yewstack/yew/pull/1961) with
  [underscore parameters](https://github.com/yewstack/yew/pull/2010) and
  [`ShouldRender` type alias](https://github.com/yewstack/yew/pull/2011) removed.

    ```rust
    pub trait Component: Sized + 'static {
      type Message: 'static;
      type Properties: Properties;

      fn create(ctx: &Context<Self>) -> Self;

      fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
      }

      fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
      }

      fn view(&self, ctx: &Context<Self>) -> Html;

      fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

      fn destroy(&mut self, ctx: &Context<Self>) {} 
    }
    ```

    `Properties` no longer have to implement `Clone` but they do need to implement `PartialEq`,
    yes...Yew gives with one hand and takes with the other 🙃.

    `Context<Self>` has a method (`link()`) which is a replacement for `ComponentLink<Self>` and
    also one that contains the component `Properties` (`props()`), this means that a component no
    longer needs to store the `ComponentLink<Self>` or `Properties`.

    Notice that, what was `change` is now called `changed`; this method is now called once the
    `Properties` have changed, and been swapped in `Context`. It was a common pattern in the old
    trait to check if the `Properties` had changed and if they had, swap them over. This was soo
    common that we even made `NeqAssign`, this is now no longer required 🎉.

    `update` and `changed` now have a sensible default implementation which can help reduce the
    boilerplate even more. 

    A simple counter component example to take the new trait for a spin:

    ```rust
    #[derive(PartialEq, Properties)]
    pub struct CounterProps {
      #[prop_or_default]
      initial_value: i64,
    }

    pub struct Counter {
      value: i64,
    }

    impl Component for Counter {
      type Message = bool;
      type Properties = CounterProps;

      fn create(ctx: &Context<Self>) -> Self {
        Self { value: ctx.props().initial_value }
      }

      fn update(&mut self, _: &Context<Self>, up: Self::Message) -> bool {
        if up {
          self.value += 1;
        } else {
          self.value -= 1;
        }
        true
      }

      fn view(&self, ctx: &Context<Self>) -> Html {
        let increment = ctx.link().callback(|_| true);
        let decrement = ctx.link().callback(|_| false);
        html! {
          <div>
            <button onclick={increment}>{ "+1" }</button>
            <button onclick={decrement}>{ "-1" }</button>
            <p>
              { self.value }
            </p>
          </div>
        }
      }
    }
    ```

  - [Remove `InputData` and `ChangeData`](https://github.com/yewstack/yew/pull/2000).

    `InputData` and `ChangeData` were too restrictive and required that the event
    [currentTarget](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget) was the
    same as the [target](https://developer.mozilla.org/en-US/docs/Web/API/Event/target). `ChangeData`
    could also panic under certain conditions.

    `oninput` and `onchange` now expect the `web_sys::InputEvent` and `web_sys::Event` respectively.

    This change also added the `TargetCast` trait to help with getting the event's target element in
    type you want, review the website documentation for more information!
  - [Add requirement for braces around non-literal property values](https://github.com/yewstack/yew/pull/1939).
  This allows Yew more flexibility when parsing in the `html!` macro which opens the door for some
  new features too!
  - [Rewrite of `agent` module to new `yew-agent` crate!](https://github.com/yewstack/yew/pull/1842).
  - [Removal of `services` module](https://github.com/yewstack/yew/issues/1841).
  
    This removes the whole module from the project and the following are the recommended replacements: 
    - `ConsoleService` - [weblog](https://crates.io/crates/weblog) or [gloo_console](https://crates.io/crates/gloo-console)
    - `DialogService` - [gloo_dialogs](https://crates.io/crates/gloo-dialogs)
    - `IntervalService` - [gloo-timers](https://crates.io/crates/gloo-timers)
    - `KeyboardService` - `onkeydown` / `onkeypress` / `onkeyup` like so:
      ```rust
      let callback = Callback::from(|e| {
        e.prevent_default();
        todo!("use `e`, like you would in service methods.");
      });

      html! {
        <input onkeydown={callback} />
      }
      ```
    - `ResizeService` - use `EventListener` from [gloo_events](https://crates.io/crates/gloo-events)
    to attach the event listener instead.
    - `StorageService` - [gloo-storage](https://crates.io/crates/gloo-storage)
    - `TimeoutService` - [gloo-timers](https://crates.io/crates/gloo-timers)
    - `WebSocketService` - [wasm-sockets](https://crates.io/crates/wasm-sockets) or
    [reqwasm](https://crates.io/crates/reqwasm)
    - `FetchService` - [reqwest](https://crates.io/crates/reqwest) or [reqwasm](https://crates.io/crates/reqwasm)
  
  - [Removal of the `format` module](https://github.com/yewstack/yew/pull/1842).

    Removal of `services` made `format` redundant and could cause conflicts or restrictions when
    using other libraries.  

  - [New Yew app entry points, multiple Yew apps enabled with `AppHandle` to control Yew app lifetime](https://github.com/yewstack/yew/pull/1825).
  
    This also removes the `App` struct, check out the new example `dyn_create_destroy_apps` to see
    these changes in action!

    Yew app entry function changes

    From:
    ```rust
    fn start_app<COMP: Component>();
    fn start_app_with_props<COMP: Component>(COMP::Properties);
    fn App::<COMP: Component>::mount(self, Element) -> ComponentLink<COMP>;
    fn App::<COMP: Component>::mount_with_props(self, Element, COMP::Properties) -> ComponentLink<COMP>;
    fn App::<COMP: Component>::mount_as_body(self) -> ComponentLink<COMP>;
    fn App::<COMP: Component>::mount_as_body_with_props(self, COMP::Properties) -> ComponentLink<COMP>;
    ```
    To:
    ```rust
    fn start_app<COMP: Component>() -> AppHandle<COMP>;
    fn start_app_with_props<COMP: Component>(COMP::Properties) -> AppHandle<COMP>;
    fn start_app_in_element<COMP: Component>(Element) -> AppHandle<COMP>;
    fn start_app_with_props_in_element<COMP: Component>(Element, COMP::Properties) -> AppHandle<COMP>;
    // mount_as_body can be replaced with this using `Default::default` or `()`
    fn start_app_with_props_as_body<COMP: Component>(COMP::Properties) -> AppHandle<COMP>;
    ```
    _Note: Not valid syntax but displays the signature in a concise way for easier comparison!_

## ✨ **0.18.0** *(2021-05-15)*

#### Changelog

- #### 🛠 Fixes

  - Fix missing redirects. [[@siku2](https://github.com/siku2), [#1640](https://github.com/yewstack/yew/pull/1640)]
  - Remove Drop bound from Task trait. [[@siku2](https://github.com/siku2), [#1627](https://github.com/yewstack/yew/pull/1627)]
  - Enable std feature for indexmap. [[@jstarry](https://github.com/jstarry), [#1709](https://github.com/yewstack/yew/pull/1709)]

- #### ⚡️ Features

  - Implicit optional attributes. [[@siku2](https://github.com/siku2), [#1637](https://github.com/yewstack/yew/pull/1637)]
  - Added callback_future_once in yewtil.(#1712). [[@fraillt](https://github.com/fraillt), [#1696](https://github.com/yewstack/yew/pull/1696)]
  - Added relevant examples section to the docs. [[@oOBoomberOo](https://github.com/oOBoomberOo), [#1695](https://github.com/yewstack/yew/pull/1695)]
  - Added missing KeyboardService re-export. [[@SOF3](https://github.com/SOF3), [#1694](https://github.com/yewstack/yew/pull/1694)]
  - Rename internal Agent structs to match Component. [[@jstarry](https://github.com/jstarry), [#1688](https://github.com/yewstack/yew/pull/1688)]
  - Add discussion link to issue selector. [[@jstarry](https://github.com/jstarry), [#1674](https://github.com/yewstack/yew/pull/1674)]
  - Update link to Material Design Components. [[@TapioT](https://github.com/TapioT), [#1662](https://github.com/yewstack/yew/pull/1662)]
  - Extract Classes to a separate macro. [[@cecton](https://github.com/cecton), [#1601](https://github.com/yewstack/yew/pull/1601)]
  - Improve the "keyed_list" example. [[@titaneric](https://github.com/titaneric), [#1650](https://github.com/yewstack/yew/pull/1650)]
  - Add documentation for component children. [[@K4rakara](https://github.com/K4rakara), [#1616](https://github.com/yewstack/yew/pull/1616)]
  - Add a macro for building properties outside of html!. [[@siku2](https://github.com/siku2), [#1599](https://github.com/yewstack/yew/pull/1599)]

## ✨ **0.17.4** *(2020-10-18)*

#### Changelog

- #### 🛠 Fixes

  - Fixed a "call stack exceeded" panic that occurred if a `Component` was updated many times [[@jstarry], [#1624](https://github.com/yewstack/yew/pull/1624)]

## ✨ **0.17.3** *(2020-08-16)*

#### Changelog

- #### ⚡️ Features

  - Added `prompt` function to `DialogService`. [[@teymour-aldridge], [#1350](https://github.com/yewstack/yew/pull/1350)]
  - Implement `From<&[T]>` where `T: AsRef<str>` for `Classes`. [[@alexschrod], [#1448](https://github.com/yewstack/yew/pull/1448)]
  - Added `batch_callback_once` to `ComponentLink`. [[@ctron], [#1463](https://github.com/yewstack/yew/pull/1463)]

- #### 🛠 Fixes

  - Properties with default type params can now have `Properties` trait derived. [[@siku2], [#1408](https://github.com/yewstack/yew/pull/1408)]
  - `html!`: Improved compile error messages for invalid list fragments. [[@siku2], [#1445](https://github.com/yewstack/yew/pull/1445)]
  - Batch component updates are processed more efficiently. [[@bakape], [#1470](https://github.com/yewstack/yew/pull/1470)]

## ✨ **0.17.2** *(2020-07-04)*

#### Changelog

- #### ⚡️ Features

  - `Key` now implements `Deref<Target = str>`. [[@faulesocke], [#1370](https://github.com/yewstack/yew/pull/1370)]

- #### 🛠 Fixes

  - Uncontrolled input values are no cleared when component renders. [[@jstarry], [#1374](https://github.com/yewstack/yew/pull/1374)]
  - Revert lazy rendering behavior introduced in `0.17.0`. Yew will render the component between each update. [[@jstarry], [#1373](https://github.com/yewstack/yew/pull/1373)]

## ✨ **0.17.1** *(2020-07-01)*

#### Changelog

- #### 🛠 Fixes

  - Fixed regression where component `rendered` lifecycle method was called before children components finish rendering. [[@jstarry], [#1360](https://github.com/yewstack/yew/pull/1360)]

## ✨ **0.17.0** *(2020-06-29)*

#### Changelog

- #### ⚡️ Features

  - Allow agents to send input messages to themselves. [[@mkawalec], [#1278](https://github.com/yewstack/yew/pull/1278)]
  - Rendering performance has been improved by [~20%](http://static.yew.rs/v0.17-benchmarks.png). [[@jstarry], [#1296](https://github.com/yewstack/yew/pull/1296), [#1309](https://github.com/yewstack/yew/pull/1309)]
  - `html!`: Elements can be specified with dynamic tag names. [[@siku2], [#1266](https://github.com/yewstack/yew/pull/1266)]

      In order to specify a dynamic tag name, wrap an expression with `@{..}`:

      ```rust
      let tag_name = "input";
      html! { <@{tag_name} value="Hello" /> }
      ```
  - HTML button element `type` can now be specified (`"submit"`, `"reset"`, or `"button"`). [[@captain-yossarian], [#1033](https://github.com/yewstack/yew/pull/1033)]
  - All global event listeners can be used as listeners (`onerror`, `onloadend`, and many more). [[@siku2], [#1244](https://github.com/yewstack/yew/pull/1242)]
  - `PartialEq` is now implemented for `VChild` when properties also implement `PartialEq`. [[@kellpossible], [#1242](https://github.com/yewstack/yew/pull/1242)]
  - Agent callbacks now accept `Into<Message>` to improve ergonomics. [[@totorigolo], [#1215](https://github.com/yewstack/yew/pull/1215)]
  - Agents can now send messages to themselves. [[@totorigolo], [#1215](https://github.com/yewstack/yew/pull/1215)]

- #### 🛠 Fixes

  - Bincode dependency version has been loosened `1.2.1` -> `1`. [[@jstarry], [#1349](https://github.com/yewstack/yew/pull/1349)]
  - Keyed list ordering algorithm has been fixed. [[@totorigolo] and [@jstarry], [#1231](https://github.com/yewstack/yew/pull/1231)]
  - `html!`: `key` and `ref` are no longer ignored for components with no properties. [[@jstarry], [#1338](https://github.com/yewstack/yew/pull/1338)]
  - `html!`: List rendering behavior is consistent no matter which syntax is chosen. [[@siku2], [#1275](https://github.com/yewstack/yew/pull/1275)]

      `html! { for node_list }` is now equivalent to `html! { node_list }` when `node_list` is a `Vec<VNode>`.

  - `KeyboardService` events can now have default behavior prevented. [[@ghpu], [#1286](https://github.com/yewstack/yew/pull/1286)]
  - Yew will check the current DOM `input` value before comparing with the desired value. [[@ShadoySV], [#1268](https://github.com/yewstack/yew/pull/1268)]
  - `html!`: Void elements (`<br/>`, `<input />`) are no longer allowed to have children. [[@kaoet], [#1217](https://github.com/yewstack/yew/pull/1217)]
  - Local agents no longer require `Input` and `Output` to implement `Serializable`. [[@mkawalec], [#1195](https://github.com/yewstack/yew/pull/1195)]

- #### 🚨 Breaking changes

  - Renders are now done lazily and will not be executed until all updates have been processed. [[@jstarry], [#1309](https://github.com/yewstack/yew/pull/1309)]
  - `ConsoleService`, `DialogService`, `IntervalService`, `RenderService`, `TimeoutService`, and `WebSocketService` methods are now static. [[@teymour-aldridge], [#1313](https://github.com/yewstack/yew/pull/1313)]
  - `html!`: `Children` no longer implements `Renderable`. [[@siku2], [#1275](https://github.com/yewstack/yew/pull/1275)]

      Replace instances of `self.props.children.render()` with `self.props.children.clone()`.

  - Yew no longer stops propagation of events by default. [[@jstarry], [#1256](https://github.com/yewstack/yew/pull/1256)]

      Event propagation is usually stopped when you have event listeners attached to nested elements and do not want the event to bubble up from where it was first captured. If your app has this behavior, you can stop propagation by calling `stop_propagation()` on the desired event.

  - The `onsubmit` listener now uses `FocusEvent` instead `Event` when using `web-sys`. [[@siku2], [#1244](https://github.com/yewstack/yew/pull/1244)]
  - The `onmousewheel` and `ontouchenter` listeners have been removed. [[@siku2], [#1244](https://github.com/yewstack/yew/pull/1244)]
  - The `ondoubleclick` listener is now named `ondblclick`. [[@siku2], [#1244](https://github.com/yewstack/yew/pull/1244)]
  - `FetchService` methods are now static. [[@teymour-aldridge], [#1235](https://github.com/yewstack/yew/pull/1235)]

      Instead of `FetchService::new().fetch(..)` you should now use `FetchService::fetch(..)`

  - The `send_message_batch` method has been removed from `AgentLink`. [[@totorigolo], [#1215](https://github.com/yewstack/yew/pull/1215)]
  - Minimum supported rust version has been bumped from `1.40.0` to `1.42.0`. [[@mkawalec], [#1195](https://github.com/yewstack/yew/pull/1195)]
  - Every agent `Reach` type is now generic. [[@mkawalec], [#1195](https://github.com/yewstack/yew/pull/1195)]

      In order to fix your app, simply append `<Self>` to the reach:

      `Reach = Context` -> `Reach = Context<Self>`
  - Removed `Global` agent because it was never implemented. [[@jstarry], [#1202](https://github.com/yewstack/yew/pull/1202)]
  - Reduced visibility of internal agent types that were not intended to be public. [[@jstarry], [#1202](https://github.com/yewstack/yew/pull/1202)]

## ✨ **0.16.2** *(2020-05-14)*

#### Changelog

- #### 🛠 Fixes

  - Fixed regression where messages sent from `Component::create` were skipped. [[@jstarry], [#1225](https://github.com/yewstack/yew/pull/1225)]

## ✨ **0.16.1** *(2020-05-14)*

#### Changelog

- #### 🛠 Fixes

  - Worker script is now loaded from absolute path. [[@domdir], [#1175](https://github.com/yewstack/yew/pull/1175)]
  - Improved `html!` macro error messages. [[@teymour-aldridge], [#1192](https://github.com/yewstack/yew/pull/1192)], [[@kaoet], [#1219](https://github.com/yewstack/yew/pull/1219)]

## ✨ **0.16** *(2020-05-09)*

#### Changelog

- #### ⚡️ Features

  - Added optional `id`, `class`, and `placeholder` properties to the `Select` component. [[@Stigjb], [#1187](https://github.com/yewstack/yew/pull/1187)]
  - Re-export `web-sys` from Yew. This allows projects to use `web-sys` without adding it to their `Cargo.toml`. [[@D4nte], [#1176](https://github.com/yewstack/yew/pull/1176)]
  - Added support for `Option` wrapped class names. [[@liquidblock], [#1085](https://github.com/yewstack/yew/pull/1085)]

    The following code is now supported:
    ```rust
    let color: &Option<String> = &self.color;
    html! { <div class=("btn", color)></div> }
    ```

  - Added `get_parent` and `get_component` methods to `ComponentLink` to allow access to parent component state. [[@jstarry], [#1151](https://github.com/yewstack/yew/pull/1151)]

- #### 🛠 Fixes

  - Fixed bug that caused html class attributes to be set to an empty string. [[@liquidblock], [#1085](https://github.com/yewstack/yew/pull/1085)]
  - Fixed `Private` worker lifecycle event sending. [[@joaquindk], [#1146](https://github.com/yewstack/yew/pull/1146)]

- #### 🚨 Breaking changes

  - Bumped minimum supported Rust version (MSRV) to 1.40.0. [[@jstarry], [#1152](https://github.com/yewstack/yew/pull/1152)]

## ✨ **0.15** *(2020-04-25)*

#### Attention!
`yew` now uses `web-sys` by default. If your project uses `web-sys`, you can now drop the `"web_sys"` feature from your yew dependency.
Don't worry `stdweb` users, we have created a new alias crate for y'all called `yew-stdweb`. In order to use it, update your `Cargo.toml` yew dependency to the following:

```toml
yew = { version = "0.15", package = "yew-stdweb" }
```

#### Dev Survey Results
Thank you to everyone that took the time to fill out the Yew Dev Survey! 🙇‍♂️

Results have been posted here: https://github.com/yewstack/yew/wiki/Dev-Survey-%5BSpring-2020%5D

#### New Chatroom
We moved from Gitter to Discord! Join us: https://discord.gg/VQck8X4

#### Changelog

- #### ⚡️ Features

  - Add support for single use callbacks (useful for `TimeoutService`). [[@lukerandall], [#1125](https://github.com/yewstack/yew/pull/1125)]
  - Updated scheduler to eagerly destroy components to avoid unnecessary renders. [[@jstarry], [#1072](https://github.com/yewstack/yew/pull/1072)]
  - Add support `key` attribute to improve rendering performance. [[@mrh0057], [#1076](https://github.com/yewstack/yew/pull/1076)]

- #### 🛠 Fixes

  - Split class names on whitespace when passed within `tuple` or `Vec`. [[@bryanjswift], [#1084](https://github.com/yewstack/yew/pull/1084)]

- #### 🚨 Breaking changes

  - The `components` module has been moved out `yew` and into `yew-components`. [[@jstarry], [#1132](https://github.com/yewstack/yew/pull/1132)]
  - Replaced `mounted` component lifecycle method with `rendered` which is called after each render. [[@jstarry], [#1072](https://github.com/yewstack/yew/pull/1072)]
  - Components must now implement the `change` method (forgetting this was a very common issue). [[@jstarry], [#1071](https://github.com/yewstack/yew/pull/1071)]
  - Yew now builds with `web-sys` by default. [[@jstarry], [#1092](https://github.com/yewstack/yew/pull/1092)]

## ✨ **0.14.3** *(2020-04-04)*

- #### 🛠 Fixes

  - Remove `html!` component validation to allow generic components. [[@mankinskin], [#1065](https://github.com/yewstack/yew/pull/1065)]
  - Improve `Debug` formatting for `VTag` and `VText`. [[@dancespiele], [#1059](https://github.com/yewstack/yew/pull/1059)]
  - Implement `Default` for `Callback`. [[@TheNeikos], [#1043](https://github.com/yewstack/yew/pull/1043)]

## ✨ **0.14.2** *(2020-03-23)*

- #### 🛠 Fixes

  - Fix issue where components were rendered out of order. [[@mrh0057] & [@jstarry], [#1051](https://github.com/yewstack/yew/pull/1051)]
  - Reset Select component correctly in Firefox / Edge. [[@kuy], [#987](https://github.com/yewstack/yew/pull/987)]

## ✨ **0.14.1** *(2020-03-14)*

- #### 🛠 Fixes

  - `Connected` message was only called for first bridge creation. [[@nicklaswj], [#1029](https://github.com/yewstack/yew/pull/1029)]

## ✨ **0.14** *(2020-03-14)*

Happy 🥧 (PI) Day! This release brings a number of bug fixes for `web-sys` apps and ergonomic improvements to the API. Huge thanks to the community for diving into the migration from `stdweb` to `web-sys` so quickly and uncovering these issues!

#### Changelog

- #### ⚡️ Features

  - Implemented `Clone` for `WebSocketStatus`. [[@kellytk], [#1023](https://github.com/yewstack/yew/pull/1023)]
  - Improved ergonomics for message APIs by accepting `Into<Msg>`. [[@captain-yossarian], [#999](https://github.com/yewstack/yew/pull/999)]
  - `html!` improved compiler messages and flexible syntax for `with props`. [[@captain-yossarian], [#960](https://github.com/yewstack/yew/pull/960)]

- #### 🛠 Fixes

  - Fixed panic in `stdweb` `ResizeService` event handling. [[@nicklaswj], [#1014](https://github.com/yewstack/yew/pull/1014)]
  - Removed build check for OS compatibility. [[@jstarry], [#1019](https://github.com/yewstack/yew/pull/1019)]
  - Fixed interval and timer usage in `web-sys` workers by updating `gloo`. [[@jstarry], [#1018](https://github.com/yewstack/yew/pull/1018)]
  - Send `Connected` message for Public agents. [[@TheNeikos], [#1007](https://github.com/yewstack/yew/pull/1007)]
  - Fixed `web-sys` Public / Private agent initialization. [[@jstarry], [#1006](https://github.com/yewstack/yew/pull/1006)]
  - Fixed websocket 'text' message handling for `web-sys` agents. [[@jstarry], [#1005](https://github.com/yewstack/yew/pull/1005)]

- #### 🚨 Breaking changes

  - `FetchError::FetchFailed` enum variant now wraps a `String` to hold the failure reason. [[@jstarry], [#1025](https://github.com/yewstack/yew/pull/1025)]
  - Message APIs now accept `Into<Msg>`, so calling `msg.into()` will cause compile errors. [[@captain-yossarian], [#999](https://github.com/yewstack/yew/pull/999)]

## ✨ **0.13.2** *(2020-03-05)*

- #### 🛠 Fixes

  - Fix clippy warning when building with `web_sys` feature. [[@jstarry], [#1001](https://github.com/yewstack/yew/pull/1001)]

## ✨ **0.13.1** *(2020-03-04)*

- #### 🛠 Fixes

  - Fix for `web-sys` version `0.3.36`. [[@detegr], [#997](https://github.com/yewstack/yew/pull/997)]

## ✨ **0.13** *(2020-03-01)*

`web-sys` support has arrived! [@daxpedda] spear-headed the effort and courageously integrated `web-sys` while maintaining support for `stdweb` through no small amount of `cfg` macro usage. We chose to continue support for apps built with `stdweb` because the dev experience is still quite a bit better _(Unfortunately `cargo-web` is incompatible with `web-sys`)_. However, the Yew team recognizes that the future of `cargo-web` of `stdweb` are uncertain. For this reason, we recommend devs start making the switch over to `web-sys` and `wasm-bindgen`. We will likely invest in improving the dev experience with these tools so that switching over is eventually a no-brainer. Please reach out with ideas and feedback for this migration through Github issues and in our Gitter chatroom!

After upgrading to v0.13, devs will now have to opt in to either `stdweb` or `web-sys` by using either the `"web_sys"` or `"std_web"` on the `yew` crate in their `Cargo.toml`.

```toml
# Choose `stdweb`
yew = { version = "0.13", features = ["std_web"] }

# Choose `web-sys`
yew = { version = "0.13", features = ["web_sys"] }
```

Lastly, take note that API docs on https://docs.rs/yew will be using the `"web_sys"` feature. For `"std_web"` docs, please visit https://docs.rs/yew-stdweb.

#### Changelog

- #### ⚡️ Features

  - Added support for building apps with `web-sys`. [[@daxpedda], [#961](https://github.com/yewstack/yew/pull/961)]
  - Properties 2.0 [[@AlephAlpha], [#975](https://github.com/yewstack/yew/pull/975)]

    Component properties are now assumed to be required unless otherwise annotated with a default value. Check out the proposal issue [#928](https://github.com/yewstack/yew/issues/928) for more details!

- #### 🛠 Fixes

  - Fixed `Component` children re-rendering bug. [[@jstarry], [#980](https://github.com/yewstack/yew/pull/980)]
  - Fixed panic when interacting with agents after receiving an agent message. [[@jstarry], [#981](https://github.com/yewstack/yew/pull/981)]
  - Fixed panic when a component with a root `VRef` node is detached. [[@jstarry], [#983](https://github.com/yewstack/yew/pull/983)]
  - Fixed annoying warning when a component with a root `VTag` node is detached. [[@jstarry], [#983](https://github.com/yewstack/yew/pull/983)]

- #### 🚨 Breaking changes

  - Changed `Properties` macro behavior. Check out the proposal issue [#928](https://github.com/yewstack/yew/issues/928) for more details! [[@AlephAlpha], [#975](https://github.com/yewstack/yew/pull/975)]
  - Cleaned up exported apis and doc visibility. [[@jstarry], [#977](https://github.com/yewstack/yew/pull/977)]
  - `ReaderService` methods now return a `Result` instead of panicking.  [[@daxpedda], [#868](https://github.com/yewstack/yew/pull/868)]
  - `FetchService` methods now return a `Result` instead of panicking.  [[@daxpedda], [#867](https://github.com/yewstack/yew/pull/867)]
  - `StorageService` methods now return a `Result` instead of panicking.  [[@daxpedda], [#827](https://github.com/yewstack/yew/pull/827)]

## ✨ **0.12** *(2020-02-16)*

- #### ⚡️ Features

  - Improved ergonomics for `html! { for .. }`. [[@jstarry], [#875](https://github.com/yewstack/yew/pull/875)]
  - Added `#[props(default = "fn_path")]` for specifying a default property value. [[@AlephAlpha], [#881](https://github.com/yewstack/yew/pull/881)]
  - Exposed the macros for creating format types. [[@ctm], [#883](https://github.com/yewstack/yew/pull/883)]
  - Added support for binary-only and text-only formats in `WebSocketService`. [[@ctm], [#851](https://github.com/yewstack/yew/pull/851)]
  - Implemented `PartialEq` for `ChildrenRenderer` to allow `children` comparison. [[@jstarry], [#916](https://github.com/yewstack/yew/pull/916)]
  - Reduced restrictions on `ComponentLink` methods to improve `Future` support. [[@jplatte], [#931](https://github.com/yewstack/yew/pull/931)]
  - Added `referrer`, `referrer_policy` and `integrity` to `FetchOptions`. [[@leo-lb], [#931](https://github.com/yewstack/yew/pull/931)]

- #### 🛠 Fixes

  - Fixed touch event listeners. [[@AlephAlpha], [#872](https://github.com/yewstack/yew/pull/872)]
  - Fixed bad behavior when setting a `ref` on a `Component`. [[@jstarry], [#913](https://github.com/yewstack/yew/pull/913)]
  - Fixed ResizeTask cancellation. [[@jstarry], [#915](https://github.com/yewstack/yew/pull/915)]

- #### 🚨 Breaking changes

  - Switched from using `failure` to `anyhow` and `thiserror` for Yew errors. [[@daxpedda], [#863](https://github.com/yewstack/yew/pull/863)]
  - Removed `cancel` method from `Task` trait in favor of relying on [`Drop`](https://doc.rust-lang.org/book/ch15-03-drop.html). [[@kakoc], [#899](https://github.com/yewstack/yew/pull/899)]
  - Renamed `NodeRef.try_into` to `NodeRef.cast` to avoid trait conflicts. [[@jstarry], [#917](https://github.com/yewstack/yew/pull/917)]

## ✨ **0.11** *(2020-01-06)*

This release aims to lay the groundwork for Yew component libraries and clean up the API for the ever elusive 1.0 release.

### Transition Guide

This release comes with a lot of breaking changes. We understand it's a hassle to update projects but the Yew team felt it was necessary to rip a few bandaids off now as we approach a 1.0 release in the (hopefully) near future. To ease the transition, here's a guide which outlines the main refactoring you will need to do for your project. (Note: all of the changes are reflected in the many example projects if you would like a proper reference example)

#### 1. Callback syntax

This is the main painful breaking change. It applies to both element listeners as well as `Component` callback properties. A good rule of thumb is that your components will now have to retain a `ComponentLink` to create callbacks on demand or initialize callbacks in your component's `create()` method.

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
let onkeydown = self.link.callback(|e: KeyDownEvent| {
    // ...
});
```

and

```
html! {
    <button onkeydown=onkeydown type="button">
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

#### 4. Properties must implement `Clone`

In yew v0.8 we removed the requirement that component properties implement `Clone`
and in this release we are adding the requirement again. This change is needed
to improve the ergonomics of nested components. The only time properties will be
cloned is when a wrapper component re-renders nested children components.

- #### ⚡️ Features

  - Added `html_nested!` macro to support nested iterable children access. [[@trivigy], [#843](https://github.com/yewstack/yew/pull/843)]
  - Added `bincode` to the list of supported formats. [[@serzhiio], [#806](https://github.com/yewstack/yew/pull/806)]
  - Added a `noop()` convenience method to `Callback` which creates a no-op callback. [[@mdtusz], [#793](https://github.com/yewstack/yew/pull/793)]
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
  - `AgentLink` now implements `Clone` which enables `Future` usage without explicit Yew framework support. [[@izissise], [#802](https://github.com/yewstack/yew/pull/802)]
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

  - Fixed handling of boolean tag attributes. [[@mrh0057], [#840](https://github.com/yewstack/yew/pull/840)]
  - Improved nested component ergonomics. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]

  ```rust
  fn view(&self) -> Html {
      html! {
          <Wrapper>
              // This is now valid. (before #780, this would cause a lifetime
              // compile error because children nodes were moved into a closure)
              <Nested on_click=&self.nested_on_click />
          </Wrapper>
      }
  }
  ```

  - Creating a `Callback` with `ComponentLink` is no longer restricted to mutable references, improving ergonomics. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `Callback` `reform` method no longer consumes self making it easier to "reverse map" a `Callback`. [[@jstarry], [#779](https://github.com/yewstack/yew/pull/779)]

  ```rust
  pub struct ListHeader {
      props: Props,
  }

  #[derive(Properties, Clone)]
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

  - Components with generic args now need to be closed with the full type path. (e.g. `html! { <Wrapper<String>></Wrapper<String>>}`) [[@jstarry], [#837](https://github.com/yewstack/yew/pull/837)]
  - Changed `VTag` listener type from `Box<dyn Listener>` to `Rc<dyn Listener>`. [[@jstarry], [#786](https://github.com/yewstack/yew/pull/786)]
  - `Properties` need to implement `Clone` again in order to improve nested component ergonomics. [[@jstarry], [#786](https://github.com/yewstack/yew/pull/786)]
  - Removed `send_future` method from `ComponentLink` since it is no longer necessary for using Futures with Yew. [[@hgzimmerman], [#799](https://github.com/yewstack/yew/pull/799)]
  - Removed generic type parameter from `Html` and all virtual node types: `VNode`, `VComp`, `VTag`, `VList`, `VText`, etc. [[@jstarry], [#783](https://github.com/yewstack/yew/pull/783)]
  - Removed support for macro magic closure syntax for element listeners. (See transition guide for how to pass a `Callback` explicitly instead). [[@jstarry], [#782](https://github.com/yewstack/yew/pull/782)]
  - Renamed `Agent` methods and event type for clarity. `handle` -> `handle_input`, `AgentUpdate` -> `AgentLifecycleEvent`, `response` -> `respond`. [[@philip-peterson], [#751](https://github.com/yewstack/yew/pull/751)]
  - The `ComponentLink` `send_back` method has been renamed to `callback` for clarity. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `ComponentLink` `send_self` and `send_self_batch` methods have been renamed to `send_message` and `send_message_batch` for clarity. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `Agent` `send_back` method has been renamed to `callback` for clarity. [[@jstarry], [#780](https://github.com/yewstack/yew/pull/780)]
  - The `VTag` `children` value type has changed from `Vec<VNode>` to `VList`. [[@jstarry], [#754](https://github.com/yewstack/yew/pull/754)]


## ✨ **0.10** *(2019-11-11)*

- #### ⚡️ Features

  - `Future` support :tada: A `Component` can update following the completion of a `Future`. Check out [this example](https://github.com/yewstack/yew/tree/v0.14.0/examples/futures) to see how it works. This approach was borrowed from a fork of Yew called [`plaster`](https://github.com/carlosdp/plaster) created by [@carlosdp]. [[@hgzimmerman], [#717](https://github.com/yewstack/yew/pull/717)]
  - Added the `agent` and `services` features so that this functionality can be disabled (useful if you are switching to using `Future`s). [[@hgzimmerman], [#684](https://github.com/yewstack/yew/pull/684)]
  - Add `ref` keyword for allowing a `Component` to have a direct reference to its rendered elements. For example, you can now easily focus an `<input>` element after mounting. [[@jstarry], [#715](https://github.com/yewstack/yew/pull/715)]

  ```rust
  use stdweb::web::html_element::InputElement;
  use stdweb::web::IHtmlElement;
  use yew::prelude::*;

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
[@alexschrod]: https://github.com/alexschrod
[@AlephAlpha]: https://github.com/AlephAlpha
[@astraw]: https://github.com/astraw
[@bakape]: https://github.com/bakape
[@bryanjswift]: https://github.com/bryanjswift
[@boydjohnson]: https://github.com/boydjohnson
[@captain-yossarian]: https://github.com/captain-yossarian
[@carlosdp]: https://github.com/carlosdp
[@charvp]: https://github.com/charvp
[@ctaggart]: https://github.com/ctaggart
[@ctm]: https://github.com/ctm
[@ctron]: https://github.com/ctron
[@domdir]: https://github.com/domdir
[@D4nte]: https://github.com/D4nte
[@dancespiele]: https://github.com/dancespiele
[@daxpedda]: https://github.com/daxpedda
[@davidkna]: https://github.com/davidkna
[@DenisKolodin]: https://github.com/DenisKolodin
[@dermetfan]: https://github.com/dermetfan
[@detegr]: https://github.com/Detegr
[@dunnock]: https://github.com/dunnock
[@faulesocke]: https://github.com/faulesocke
[@hgzimmerman]: https://github.com/hgzimmerman
[@izissise]: https://github.com/izissise
[@joaquindk]: https://github.com/joaquindk
[@jplatte]: https://github.com/jplatte
[@jstarry]: https://github.com/jstarry
[@kakoc]: https://github.com/kakoc
[@kaoet]: https://github.com/kaoet
[@kellytk]: https://github.com/kellytk
[@kuy]: https://github.com/kuy
[@leo-lb]: https://github.com/leo-lb
[@liquidblock]: https://github.com/liquidblock
[@lizhaoxian]: https://github.com/lizhaoxian
[@lukerandall]: https://github.com/lukerandall
[@mankinskin]: https://github.com/mankinskin
[@mdtusz]: https://github.com/mdtusz
[@mkawalec]: https://github.com/mkawalec
[@mrh0057]: https://github.com/mrh0057
[@nicklaswj]: https://github.com/nicklaswj
[@philip-peterson]: https://github.com/philip-peterson
[@serzhiio]: https://github.com/serzhiio
[@siku2]: https://github.com/siku2
[@Stigjb]: https://github.com/Stigjb
[@stkevintan]: https://github.com/stkevintan
[@TheNeikos]: https://github.com/TheNeikos
[@teymour-aldridge]: https://github.com/teymour-aldridge
[@tiziano88]: https://github.com/tiziano88
[@trivigy]: https://github.com/trivigy
[@totorigolo]: https://github.com/totorigolo
[@Wodann]: https://github.com/Wodann
