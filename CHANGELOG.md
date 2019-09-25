# Changelog

## ✨ **0.10** *(TBD)*

- #### ⚡️ Features

- #### 🛠 Fixes

- #### 🚨 Breaking changes

## ✨ **0.9** *(2019-09-24)*

- #### ⚡️ Features

  - Components can now accept children in the `html!` macro. [[@jstarry], [#589](https://github.com/yewstack/yew/pull/589)]

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

  - Iterators can now be rendered in the `html!` macro without using the `for` keyword. [[@hgzimmerman], [#622](https://github.com/yewstack/yew/pull/622)]

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
[@charvp]: https://github.com/charvp
[@davidkna]: https://github.com/davidkna
[@DenisKolodin]: https://github.com/DenisKolodin
[@dermetfan]: https://github.com/dermetfan
[@hgzimmerman]: https://github.com/hgzimmerman
[@jstarry]: https://github.com/jstarry
[@kellytk]: https://github.com/kellytk
[@tiziano88]: https://github.com/tiziano88
[@totorigolo]: https://github.com/totorigolo
[@Wodann]: https://github.com/Wodann
