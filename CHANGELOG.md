# Changelog

## ✨ yew **0.20.0** *(2022-11-xx)*

#### Changelog

- #### 🛠 Fixes

  - Fix onsubmit event type in docs. [[@Allan](https://github.com/Allan), [#2926](https://github.com/yewstack/yew/pull/2926)]
  - Fix issues with tuples in closing tag. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2886](https://github.com/yewstack/yew/pull/2886)]
  - Fix checked property being reset. [[@WorldSEnder](https://github.com/WorldSEnder), [#2907](https://github.com/yewstack/yew/pull/2907)]
  - Fix VList Stream in SSR. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2801](https://github.com/yewstack/yew/pull/2801)]
  - Fixed `NodeRef` not being implicitly cloned with components. [[@wdcocq](https://github.com/wdcocq), [#2775](https://github.com/yewstack/yew/pull/2775)]
  - Attributes: Fix apply_diff_index_maps. [[@Dietmar Maurer](https://github.com/Dietmar Maurer), [#2653](https://github.com/yewstack/yew/pull/2653)]
  - Fix bubbling of events originating in shadow dom. [[@WorldSEnder](https://github.com/WorldSEnder), [#2627](https://github.com/yewstack/yew/pull/2627)]
  - Fix some Hook edge cases. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2592](https://github.com/yewstack/yew/pull/2592)]
  - Fix issue with node refs and hydration. [[@WorldSEnder](https://github.com/WorldSEnder), [#2597](https://github.com/yewstack/yew/pull/2597)]
  - Fix macro hygiene issues. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2585](https://github.com/yewstack/yew/pull/2585)]
  - Fix casing of dynamic tags. [[@WorldSEnder](https://github.com/WorldSEnder), [#2578](https://github.com/yewstack/yew/pull/2578)]
  - Automatically convert closure to callback for component properties. [[@Finn Bear](https://github.com/Finn Bear), [#2554](https://github.com/yewstack/yew/pull/2554)]
  - Fix a problem with NodeRefs and VTags, ref. [[@WorldSEnder](https://github.com/WorldSEnder), [#2279](https://github.com/yewstack/yew/pull/2279)]
  - Fix defaulted type parameter.. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2284](https://github.com/yewstack/yew/pull/2284)]
  - Use Ref::filter_map if rustc is later than 1.63. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2904](https://github.com/yewstack/yew/pull/2904)]
  - Evaluate props in the order they're defined. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2887](https://github.com/yewstack/yew/pull/2887)]
  - Context: Avoid storing a copy of children. [[@Dietmar Maurer](https://github.com/Dietmar Maurer), [#2885](https://github.com/yewstack/yew/pull/2885)]
  - Various improvements to Classes, oriented around reducing allocations. [[@Nathan West](https://github.com/Nathan West), [#2870](https://github.com/yewstack/yew/pull/2870)]
  - Resume Suspension upon unmount. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2874](https://github.com/yewstack/yew/pull/2874)]
  - Make fn update() re-render the component by default. [[@Cecile Tonglet](https://github.com/Cecile Tonglet), [#2786](https://github.com/yewstack/yew/pull/2786)]
  - Do not detach child elements if parent element is about to be detached. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2420](https://github.com/yewstack/yew/pull/2420)]
  - remove some unsafes by using atomics. [[@WorldSEnder](https://github.com/WorldSEnder), [#2186](https://github.com/yewstack/yew/pull/2186)]
  - `use_prepared_state` & `use_transitive_state`. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2650](https://github.com/yewstack/yew/pull/2650)]
  - Silence some warnings from derive(Properties). [[@WorldSEnder](https://github.com/WorldSEnder), [#2266](https://github.com/yewstack/yew/pull/2266)]
  - onsubmit should be a SubmitEvent. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2816](https://github.com/yewstack/yew/pull/2816)]

- #### ⚡️ Features

  - Add VNode::from_html_unchecked. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2842](https://github.com/yewstack/yew/pull/2842)]
  - Make Yew lints opt-in. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2882](https://github.com/yewstack/yew/pull/2882)]
  - Allow skipping a callback when reforming. [[@Jens Reimann](https://github.com/Jens Reimann), [#2864](https://github.com/yewstack/yew/pull/2864)]
  - Polled SSR Stream. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2824](https://github.com/yewstack/yew/pull/2824)]
  - Add send_stream method for Scope. [[@laizy](https://github.com/laizy), [#2619](https://github.com/yewstack/yew/pull/2619)]
  - Allow functions returning unit in `use_effect`. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2849](https://github.com/yewstack/yew/pull/2849)]
  - Configurable Runtime. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2772](https://github.com/yewstack/yew/pull/2772)]
  - Pinned Channels. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2811](https://github.com/yewstack/yew/pull/2811)]
  - Bind to properties instead of attributes by default. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2819](https://github.com/yewstack/yew/pull/2819)]
  - Convert nightly from a feature flag to a compiler flag. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2827](https://github.com/yewstack/yew/pull/2827)]
  - Reduce SSR Buffers in VList. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2826](https://github.com/yewstack/yew/pull/2826)]
  - Allow keywords after dash in element and attribute names. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2820](https://github.com/yewstack/yew/pull/2820)]
  - Replace custom logging by tracing. [[@WorldSEnder](https://github.com/WorldSEnder), [#2814](https://github.com/yewstack/yew/pull/2814)]
  - Implement sleep and interval for Yew Platform. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2784](https://github.com/yewstack/yew/pull/2784)]
  - Remove component NodeRef. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2783](https://github.com/yewstack/yew/pull/2783)]
  - Prepared States dependency should be Reference Counted. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2769](https://github.com/yewstack/yew/pull/2769)]
  - Document features automatically.. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2780](https://github.com/yewstack/yew/pull/2780)]
  - Streamed SSR Response. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2697](https://github.com/yewstack/yew/pull/2697)]
  - Nightly features. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2743](https://github.com/yewstack/yew/pull/2743)]
  - Allow VNode props to be converted to Children.. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2749](https://github.com/yewstack/yew/pull/2749)]
  - Redo derive(Properties), take 2. [[@WorldSEnder](https://github.com/WorldSEnder), [#2729](https://github.com/yewstack/yew/pull/2729)]
  - `Callback::reform()` should return `Callback<T, OUT>`. [[@orzogc](https://github.com/orzogc), [#2719](https://github.com/yewstack/yew/pull/2719)]
  - Span hygiene and editor UX. [[@WorldSEnder](https://github.com/WorldSEnder), [#2702](https://github.com/yewstack/yew/pull/2702)]
  - Block props update during hydration. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2665](https://github.com/yewstack/yew/pull/2665)]
  - Point to `callback_future` in `callback` docs. [[@Shadlock0133](https://github.com/Shadlock0133), [#2674](https://github.com/yewstack/yew/pull/2674)]
  - Change access to VList children to a wrapper. [[@WorldSEnder](https://github.com/WorldSEnder), [#2673](https://github.com/yewstack/yew/pull/2673)]
  - Partially undo #2673, different approach for the DerefMut impl of VList. [[@WorldSEnder](https://github.com/WorldSEnder), [#2692](https://github.com/yewstack/yew/pull/2692)]
  - Rework a bunch of cfg(feature) flags to be more principled. [[@WorldSEnder](https://github.com/WorldSEnder), [#2666](https://github.com/yewstack/yew/pull/2666)]
  - Delay Hydration second render until all assistive nodes have been removed. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2629](https://github.com/yewstack/yew/pull/2629)]
  - Allow to consume deps in use_callback. [[@Jet Li](https://github.com/Jet Li), [#2617](https://github.com/yewstack/yew/pull/2617)]
  - Add `use_future` hook to make consuming futures as suspense easier. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2609](https://github.com/yewstack/yew/pull/2609)]
  - Add the ability to use non-literal string as attribute names. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2593](https://github.com/yewstack/yew/pull/2593)]
  - Introduce a dedicated use_force_update hook. [[@WorldSEnder](https://github.com/WorldSEnder), [#2586](https://github.com/yewstack/yew/pull/2586)]
  - Impl ImplicitClone for Rc<T> where T: Sized. [[@Nano](https://github.com/Nano), [#2594](https://github.com/yewstack/yew/pull/2594)]
  - SSR Hydration. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2552](https://github.com/yewstack/yew/pull/2552)]
  - Add use_callback hook. [[@Jet Li](https://github.com/Jet Li), [#2566](https://github.com/yewstack/yew/pull/2566)]
  - Introduce additional information in SSR artifact to facilitate Hydration. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2540](https://github.com/yewstack/yew/pull/2540)]
  - Scoped event handlers. [[@WorldSEnder](https://github.com/WorldSEnder), [#2510](https://github.com/yewstack/yew/pull/2510)]
  - An ever Increasing Component ID. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2537](https://github.com/yewstack/yew/pull/2537)]
  - Prevents Fallback UI from becoming suspended. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2532](https://github.com/yewstack/yew/pull/2532)]
  - `#[cfg(feature = "render")]` and `yew::Renderer`. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2498](https://github.com/yewstack/yew/pull/2498)]
  - Introduce explicit internal datastructures modeling dom state. [[@WorldSEnder](https://github.com/WorldSEnder), [#2330](https://github.com/yewstack/yew/pull/2330)]
  - Improve AnyScope API. [[@Aaron Erhardt](https://github.com/Aaron Erhardt), [#2445](https://github.com/yewstack/yew/pull/2445)]
  - Automatic Message Batching. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2421](https://github.com/yewstack/yew/pull/2421)]
  - Add Other variant to the ListenerKind. [[@Alexander Mescheryakov](https://github.com/Alexander Mescheryakov), [#2417](https://github.com/yewstack/yew/pull/2417)]
  - Function Components & Hooks V2. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2401](https://github.com/yewstack/yew/pull/2401)]
  - Add ContextHandle in yew::prelude. [[@Anuvrat Singh](https://github.com/Anuvrat Singh), [#2372](https://github.com/yewstack/yew/pull/2372)]
  - Separate scheduler rendered call from create and render. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2374](https://github.com/yewstack/yew/pull/2374)]
  - Update to edition 2021. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2354](https://github.com/yewstack/yew/pull/2354)]
  - Server-side Rendering (without hydration). [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2335](https://github.com/yewstack/yew/pull/2335)]
  - Make BaseComponent Sealed.. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2359](https://github.com/yewstack/yew/pull/2359)]
  - Remove start_app_as_body.. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2346](https://github.com/yewstack/yew/pull/2346)]
  - Bump minimal supported rust version (MSRV) to 1.56. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2334](https://github.com/yewstack/yew/pull/2334)]
  - Suspense Support. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2212](https://github.com/yewstack/yew/pull/2212)]
  - make layout testing code public. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2310](https://github.com/yewstack/yew/pull/2310)]
  - Refactor and simplify `Callback`. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2301](https://github.com/yewstack/yew/pull/2301)]
  - Add pending event listener on the VTag. [[@Alexander Mescheryakov](https://github.com/Alexander Mescheryakov), [#2300](https://github.com/yewstack/yew/pull/2300)]
  - constify VList::new. [[@Alexander Mescheryakov](https://github.com/Alexander Mescheryakov), [#2293](https://github.com/yewstack/yew/pull/2293)]
  - Allow `function_component` creation based on function name. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2292](https://github.com/yewstack/yew/pull/2292)]
  - Implement IntoPropValue for Rc<str>. [[@Zachary Stewart](https://github.com/Zachary Stewart), [#2285](https://github.com/yewstack/yew/pull/2285)]
  - Raw field names in property structs. [[@WorldSEnder](https://github.com/WorldSEnder), [#2273](https://github.com/yewstack/yew/pull/2273)]

## ✨ yew-router **0.17.0** *(2022-11-xx)*

#### Changelog

- #### 🛠 Fixes

  - Fix basename handling in router. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2297](https://github.com/yewstack/yew/pull/2297)]

- #### ⚡️ Features

  - Simple `NodeRef` passing to `<Link>` for yew-router. [[@Athan Clark](https://github.com/Athan Clark), [#2877](https://github.com/yewstack/yew/pull/2877)]
  - Make Switch to accept a closure as render function directly. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2659](https://github.com/yewstack/yew/pull/2659)]
  - `#[cfg(feature = "render")]` and `yew::Renderer`. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2498](https://github.com/yewstack/yew/pull/2498)]
  - Includes query parameters in rendered Link component. [[@Yuki Kodama](https://github.com/Yuki Kodama), [#2464](https://github.com/yewstack/yew/pull/2464)]
  - Update to edition 2021. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2354](https://github.com/yewstack/yew/pull/2354)]
  - Support named wildcards when deriving Routable.. [[@Jonathan Bailey](https://github.com/Jonathan Bailey), [#2345](https://github.com/yewstack/yew/pull/2345)]
  - Add HashRouter, basename and use gloo-history. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2239](https://github.com/yewstack/yew/pull/2239)]


## ✨ yew-agent **0.2.0** *(2022-11-xx)*

#### Changelog

- #### ⚡️ Features

  - add `use_bridge` docs. [[@Shrey Sudhir](https://github.com/Shrey Sudhir), [#2722](https://github.com/yewstack/yew/pull/2722)]
  - Update to edition 2021. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2354](https://github.com/yewstack/yew/pull/2354)]
  - Move yew-agent to gloo. [[@Muhammad Hamza](https://github.com/Muhammad Hamza), [#2326](https://github.com/yewstack/yew/pull/2326)]
  - Implement PrivateAgent. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2318](https://github.com/yewstack/yew/pull/2318)]
  - Remove context & job agent. [[@Kaede Hoshikawa](https://github.com/Kaede Hoshikawa), [#2295](https://github.com/yewstack/yew/pull/2295)]

## ✨ yew **0.19.0** *(2021-11-26)*

#### Changelog

- #### 🛠 Fixes

  - Attempt to fix recursion on display. [[@mibes](https://github.com/mibes), [#2149](https://github.com/yewstack/yew/pull/2149)]
  - Fix default passive option. [[@mc1098](https://github.com/mc1098), [#2111](https://github.com/yewstack/yew/pull/2111)]
  - Fix trybuild. [[@mc1098](https://github.com/mc1098), [#2103](https://github.com/yewstack/yew/pull/2103)]
  - Fix event handler during capture phase. [[@mc1098](https://github.com/mc1098), [#2062](https://github.com/yewstack/yew/pull/2062)]
  - Fix `VTag` reuse to reset ancestor `NodeRef`. [[@mc1098](https://github.com/mc1098), [#2030](https://github.com/yewstack/yew/pull/2030)]
  - Fix IntoEventCallback over IntoPropValue. [[@mc1098](https://github.com/mc1098), [#2025](https://github.com/yewstack/yew/pull/2025)]
  - Remove underscore prefix on fn parameters. [[@mc1098](https://github.com/mc1098), [#2010](https://github.com/yewstack/yew/pull/2010)]
  - Fix rust-analyzer #[derive(Properties)] warnings. [[@KarlitosVII](https://github.com/KarlitosVII), [#2007](https://github.com/yewstack/yew/pull/2007)]
  - Fix clippy lints from 1.54.0. [[@Xavientois](https://github.com/Xavientois), [#1976](https://github.com/yewstack/yew/pull/1976)]
  - Fix scheduler main queue delay (#1953). [[@intendednull](https://github.com/intendednull), [#1954](https://github.com/yewstack/yew/pull/1954)]
  - Fix case warning on derived properties. [[@nitnelave](https://github.com/nitnelave), [#1929](https://github.com/yewstack/yew/pull/1929)]
  - yew-macro: fix inability to set the autoplay atribute. [[@bakape](https://github.com/bakape), [#1866](https://github.com/yewstack/yew/pull/1866)]
  - Fix duplicate `with props` error messages.. [[@teymour-aldridge](https://github.com/teymour-aldridge), [#1730](https://github.com/yewstack/yew/pull/1730)]
  - Remove extra braces in html_nested macro. [[@Madoshakalaka](https://github.com/Madoshakalaka), [#2169](https://github.com/yewstack/yew/pull/2169)]
  - Remove unused punct field from props. [[@Xavientois](https://github.com/Xavientois), [#1969](https://github.com/yewstack/yew/pull/1969)]

- #### ⚡️ Features

  - Check event bubbling cancellation at each step of propagation. [[@rjmac](https://github.com/rjmac), [#2191](https://github.com/yewstack/yew/pull/2191)]
  - Add possibility to cancel bubbling. [[@voidpumpkin](https://github.com/voidpumpkin), [#2172](https://github.com/yewstack/yew/pull/2172)]
  - Add the ability to add child nodes conditionally in `html!`. [[@cecton](https://github.com/cecton), [#1609](https://github.com/yewstack/yew/pull/1609)]
  - Add basic lints to the HTML macro.. [[@teymour-aldridge](https://github.com/teymour-aldridge), [#1748](https://github.com/yewstack/yew/pull/1748)]
  - Refactor use ref hooks. [[@mc1098](https://github.com/mc1098), [#2093](https://github.com/yewstack/yew/pull/2093)]
  - Implementation of portals. [[@WorldSEnder](https://github.com/WorldSEnder), [#2147](https://github.com/yewstack/yew/pull/2147)]
  - Allow Classes properties to be created from string literals. [[@jplatte](https://github.com/jplatte), [#2141](https://github.com/yewstack/yew/pull/2141)]
  - Use functions from `gloo_utils` instead of re-implementing them. [[@hamza1311](https://github.com/hamza1311), [#2124](https://github.com/yewstack/yew/pull/2124)]
  - Reliable `use_reducer` dispatch and `use_state` setter. [[@futursolo](https://github.com/futursolo), [#2126](https://github.com/yewstack/yew/pull/2126)]
  - Add custom type for attribute values. [[@hamza1311](https://github.com/hamza1311), [#1994](https://github.com/yewstack/yew/pull/1994)]
  - Remove trailing semicolon in macro used in expression position. [[@vrmiguel](https://github.com/vrmiguel), [#2127](https://github.com/yewstack/yew/pull/2127)]
  - Add set_if_neq for UseStateHandle. [[@voidpumpkin](https://github.com/voidpumpkin), [#2109](https://github.com/yewstack/yew/pull/2109)]
  - Add safe first_node fn. [[@mc1098](https://github.com/mc1098), [#2094](https://github.com/yewstack/yew/pull/2094)]
  - impl PartialEq for `UseStateHandle` and `UseReducerHandle`. [[@hamza1311](https://github.com/hamza1311), [#2092](https://github.com/yewstack/yew/pull/2092)]
  - Remove `web_sys` re-export. [[@mc1098](https://github.com/mc1098), [#2084](https://github.com/yewstack/yew/pull/2084)]
  - Use into_prop_value to convert str prop to Option<String>. [[@Xavientois](https://github.com/Xavientois), [#2080](https://github.com/yewstack/yew/pull/2080)]
  - Component lifecycle scheduler optimizations. [[@bakape](https://github.com/bakape), [#2065](https://github.com/yewstack/yew/pull/2065)]
  - Update dependencies. [[@mc1098](https://github.com/mc1098), [#2064](https://github.com/yewstack/yew/pull/2064)]
  - Add support for missing boolean attributes. [[@mc1098](https://github.com/mc1098), [#2051](https://github.com/yewstack/yew/pull/2051)]
  - Add fully qualified primitives in proc macro. [[@mc1098](https://github.com/mc1098), [#2037](https://github.com/yewstack/yew/pull/2037)]
  - Remove 'static lifetime from hook init function. [[@mc1098](https://github.com/mc1098), [#2039](https://github.com/yewstack/yew/pull/2039)]
  - Add "struct update" syntax to pass props to component (`..props` instead of `with props`). [[@Xavientois](https://github.com/Xavientois), [#2024](https://github.com/yewstack/yew/pull/2024)]
  - Add `no_implicit_prelude` to proc macro tests. [[@mc1098](https://github.com/mc1098), [#2033](https://github.com/yewstack/yew/pull/2033)]
  - Dev/listener multiplexer. [[@bakape](https://github.com/bakape), [#1542](https://github.com/yewstack/yew/pull/1542)]
  - Remove ShouldRender type alias. [[@mc1098](https://github.com/mc1098), [#2011](https://github.com/yewstack/yew/pull/2011)]
  - Components v2 (2). [[@hamza1311](https://github.com/hamza1311), [#1961](https://github.com/yewstack/yew/pull/1961)]
  - Remove InputData & ChangeData. [[@mc1098](https://github.com/mc1098), [#2000](https://github.com/yewstack/yew/pull/2000)]
  - Support const generics in `#[derive(Properties)]`. [[@maciejhirsz](https://github.com/maciejhirsz), [#1978](https://github.com/yewstack/yew/pull/1978)]
  - Add shorthand syntax for props. [[@Xavientois](https://github.com/Xavientois), [#1970](https://github.com/yewstack/yew/pull/1970)]
  - Static attribute lists. [[@bakape](https://github.com/bakape), [#1962](https://github.com/yewstack/yew/pull/1962)]
  - yew/vlist: optimize diffing and patching. [[@bakape](https://github.com/bakape), [#1555](https://github.com/yewstack/yew/pull/1555)]
  - Add requirement for braces around most props. [[@Xavientois](https://github.com/Xavientois), [#1939](https://github.com/yewstack/yew/pull/1939)]
  - Optimize VTag construction, memory footprint and patching. [[@bakape](https://github.com/bakape), [#1947](https://github.com/yewstack/yew/pull/1947)]
  - Refactor and cleanup codebase. [[@hamza1311](https://github.com/hamza1311), [#1842](https://github.com/yewstack/yew/pull/1842)]
  - Helper to build changelog. [[@cecton](https://github.com/cecton), [#1845](https://github.com/yewstack/yew/pull/1845)]
  - Implicit optional attributes. [[@siku2](https://github.com/siku2), [#1637](https://github.com/yewstack/yew/pull/1637)]
  - yew: reduce scheduler call indirection. [[@bakape](https://github.com/bakape), [#1903](https://github.com/yewstack/yew/pull/1903)]
  - Change match statement to if. [[@Xavientois](https://github.com/Xavientois), [#1884](https://github.com/yewstack/yew/pull/1884)]
  - Optimize vtag construction. [[@bakape](https://github.com/bakape), [#1867](https://github.com/yewstack/yew/pull/1867)]
  - Apply Clippy lints.. [[@teymour-aldridge](https://github.com/teymour-aldridge), [#1863](https://github.com/yewstack/yew/pull/1863)]
  - Change the app struct to be a real handle to an Yew app instance and make it possible to destroy a running app. [[@nicklaswj](https://github.com/nicklaswj), [#1825](https://github.com/yewstack/yew/pull/1825)]
  - Bring context to standard components. [[@Diggsey](https://github.com/Diggsey), [#1835](https://github.com/yewstack/yew/pull/1835)]
  - Upgraded Hook API (2). [[@hamza1311](https://github.com/hamza1311), [#1780](https://github.com/yewstack/yew/pull/1780)]
  - Store hook state in a mutable scoped-TLS. [[@Diggsey](https://github.com/Diggsey), [#1831](https://github.com/yewstack/yew/pull/1831)]
  - Remove unnecessary allocation from `AnyScope`. [[@Diggsey](https://github.com/Diggsey), [#1830](https://github.com/yewstack/yew/pull/1830)]
  - Added missing licenses to Cargo.toml files and updated to use SPDX syntax. [[@jbg](https://github.com/jbg), [#1822](https://github.com/yewstack/yew/pull/1822)]
  - Update Rust version for macro tests to 1.51 & enable const generics tests. [[@hamza1311](https://github.com/hamza1311), [#1801](https://github.com/yewstack/yew/pull/1801)]
  - Allow the use of Rust keywords for element names. [[@siku2](https://github.com/siku2), [#1772](https://github.com/yewstack/yew/pull/1772)]
  - Refactor html tag peeking. [[@lukechu10](https://github.com/lukechu10), [#1738](https://github.com/yewstack/yew/pull/1738)]
  - Generic functional components. [[@lukechu10](https://github.com/lukechu10), [#1756](https://github.com/yewstack/yew/pull/1756)]
  - Add support for the unit struct in Properties derive. [[@Xavientois](https://github.com/Xavientois), [#1752](https://github.com/yewstack/yew/pull/1752)]
  - Rip out stdweb. [[@philip-peterson](https://github.com/philip-peterson), [#1697](https://github.com/yewstack/yew/pull/1697)]

## ✨ yew-router **0.16.0** *(2021-11-26)*

#### Changelog

- #### 🛠 Fixes

  - Fix Some Router Behaviour. [[@futursolo](https://github.com/futursolo), [#2107](https://github.com/yewstack/yew/pull/2107)]
  - Fix multiple field enum tokens. [[@mc1098](https://github.com/mc1098), [#1988](https://github.com/yewstack/yew/pull/1988)]
  - Fix clippy lints from 1.54.0. [[@Xavientois](https://github.com/Xavientois), [#1976](https://github.com/yewstack/yew/pull/1976)]
  - Fix router. [[@hamza1311](https://github.com/hamza1311), [#1856](https://github.com/yewstack/yew/pull/1856)]
  - update nom 6.1.2 and fix compile errors.. [[@higumachan](https://github.com/higumachan), [#1806](https://github.com/yewstack/yew/pull/1806)]

- #### ⚡️ Features

  - Add missing router docs: Redirect, Nested Router, and Path Segament capturing. [[@Madoshakalaka](https://github.com/Madoshakalaka), [#2192](https://github.com/yewstack/yew/pull/2192)]
  - use base url for href of links. [[@WorldSEnder](https://github.com/WorldSEnder), [#2177](https://github.com/yewstack/yew/pull/2177)]
  - Use functions from `gloo_utils` instead of re-implementing them. [[@hamza1311](https://github.com/hamza1311), [#2124](https://github.com/yewstack/yew/pull/2124)]
  - Update Yew Router as per #2113. [[@futursolo](https://github.com/futursolo), [#2118](https://github.com/yewstack/yew/pull/2118)]
  - Update dependencies. [[@mc1098](https://github.com/mc1098), [#2064](https://github.com/yewstack/yew/pull/2064)]
  - Add "replace route" call as a companion to "push route". [[@rjmac](https://github.com/rjmac), [#2023](https://github.com/yewstack/yew/pull/2023)]
  - Add shorthand syntax for props. [[@Xavientois](https://github.com/Xavientois), [#1970](https://github.com/yewstack/yew/pull/1970)]
  - Add requirement for braces around most props. [[@Xavientois](https://github.com/Xavientois), [#1939](https://github.com/yewstack/yew/pull/1939)]
  - Clean-up and optimize router a little bit. [[@hamza1311](https://github.com/hamza1311), [#1869](https://github.com/yewstack/yew/pull/1869)]
  - Apply Clippy lints.. [[@teymour-aldridge](https://github.com/teymour-aldridge), [#1863](https://github.com/yewstack/yew/pull/1863)]
  - Rewrite router. [[@hamza1311](https://github.com/hamza1311), [#1791](https://github.com/yewstack/yew/pull/1791)]
  - Added missing licenses to Cargo.toml files and updated to use SPDX syntax. [[@jbg](https://github.com/jbg), [#1822](https://github.com/yewstack/yew/pull/1822)]
  - Update Rust version for macro tests to 1.51 & enable const generics tests. [[@hamza1311](https://github.com/hamza1311), [#1801](https://github.com/yewstack/yew/pull/1801)]
  - Add support for the unit struct in Properties derive. [[@Xavientois](https://github.com/Xavientois), [#1752](https://github.com/yewstack/yew/pull/1752)]

## ✨ yew-agent **0.1.0** *(2021-11-26)*

#### Changelog

- #### 🛠 Fixes

  - Fix unmaintained anymap dependency. [[@mc1098](https://github.com/mc1098), [#2071](https://github.com/yewstack/yew/pull/2071)]
  - Fix clippy lints from 1.54.0. [[@Xavientois](https://github.com/Xavientois), [#1976](https://github.com/yewstack/yew/pull/1976)]
  - Fix crash in link to destroyed agents. [[@kristoff3r](https://github.com/kristoff3r), [#1827](https://github.com/yewstack/yew/pull/1827)]

- #### ⚡️ Features

  - allow web worker resource to be relative. [[@astraw](https://github.com/astraw), [#2086](https://github.com/yewstack/yew/pull/2086)]
  - `use_bridge` hook for agents. [[@futursolo](https://github.com/futursolo), [#2125](https://github.com/yewstack/yew/pull/2125)]
  - Use functions from `gloo_utils` instead of re-implementing them. [[@hamza1311](https://github.com/hamza1311), [#2124](https://github.com/yewstack/yew/pull/2124)]
  - yew-agent: add missing web-sys features. [[@astraw](https://github.com/astraw), [#2085](https://github.com/yewstack/yew/pull/2085)]
  - Update dependencies. [[@mc1098](https://github.com/mc1098), [#2064](https://github.com/yewstack/yew/pull/2064)]
  - Drop Private worker handler when bridge is dropped. [[@FrancisMurillo](https://github.com/FrancisMurillo), [#1944](https://github.com/yewstack/yew/pull/1944)]

## ✨ yew **0.18.0** *(2021-05-15)*

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

## ✨ yew-router **0.15.0** *(2021-05-15)*

- #### ⚡️ Features
  - None
- #### 🛠 Fixes
  - None
- #### 🚨 Breaking changes
  - `RouterButton` now prevents default events per default @TheNeikos

## ✨ yew **0.17.4** *(2020-10-18)*

#### Changelog

- #### 🛠 Fixes

  - Fixed a "call stack exceeded" panic that occurred if a `Component` was updated many times [[@jstarry], [#1624](https://github.com/yewstack/yew/pull/1624)]

## ✨ yew **0.17.3** *(2020-08-16)*

#### Changelog

- #### ⚡️ Features

  - Added `prompt` function to `DialogService`. [[@teymour-aldridge], [#1350](https://github.com/yewstack/yew/pull/1350)]
  - Implement `From<&[T]>` where `T: AsRef<str>` for `Classes`. [[@alexschrod], [#1448](https://github.com/yewstack/yew/pull/1448)]
  - Added `batch_callback_once` to `ComponentLink`. [[@ctron], [#1463](https://github.com/yewstack/yew/pull/1463)]

- #### 🛠 Fixes

  - Properties with default type params can now have `Properties` trait derived. [[@siku2], [#1408](https://github.com/yewstack/yew/pull/1408)]
  - `html!`: Improved compile error messages for invalid list fragments. [[@siku2], [#1445](https://github.com/yewstack/yew/pull/1445)]
  - Batch component updates are processed more efficiently. [[@bakape], [#1470](https://github.com/yewstack/yew/pull/1470)]

## ✨ yew **0.17.2** *(2020-07-04)*

#### Changelog

- #### ⚡️ Features

  - `Key` now implements `Deref<Target = str>`. [[@faulesocke], [#1370](https://github.com/yewstack/yew/pull/1370)]

- #### 🛠 Fixes

  - Uncontrolled input values are no cleared when component renders. [[@jstarry], [#1374](https://github.com/yewstack/yew/pull/1374)]
  - Revert lazy rendering behavior introduced in `0.17.0`. Yew will render the component between each update. [[@jstarry], [#1373](https://github.com/yewstack/yew/pull/1373)]

## ✨ yew **0.17.1** *(2020-07-01)*

#### Changelog

- #### 🛠 Fixes

  - Fixed regression where component `rendered` lifecycle method was called before children components finish rendering. [[@jstarry], [#1360](https://github.com/yewstack/yew/pull/1360)]

## ✨ yew-router **0.14.0** *(2020-06-30)*

- #### ⚡️ Features
  - None 
- #### 🛠 Fixes
  - None 
- #### 🚨 Breaking changes
  - The `unit_state` module has been removed. 
  - Bump `yew` version to `0.17`.

## ✨ yew **0.17.0** *(2020-06-29)*

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

## ✨ yew **0.16.2** *(2020-05-14)*

#### Changelog

- #### 🛠 Fixes

  - Fixed regression where messages sent from `Component::create` were skipped. [[@jstarry], [#1225](https://github.com/yewstack/yew/pull/1225)]

## ✨ yew **0.16.1** *(2020-05-14)*

#### Changelog

- #### 🛠 Fixes

  - Worker script is now loaded from absolute path. [[@domdir], [#1175](https://github.com/yewstack/yew/pull/1175)]
  - Improved `html!` macro error messages. [[@teymour-aldridge], [#1192](https://github.com/yewstack/yew/pull/1192)], [[@kaoet], [#1219](https://github.com/yewstack/yew/pull/1219)]

## ✨ yew-router **0.13.0** *(2020-05-12)*

- #### 🚨 Breaking changes
  - Bump `yew` version to `0.16`.

## ✨ yew **0.16** *(2020-05-09)*

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

## ✨ yew-router **0.12.1** *(2020-04-26)*
- #### 🛠 Fixes
  - Fix infinite rerender bug in 'Router' component. (Thanks @dancespiele)

## ✨ yew **0.15** *(2020-04-25)*

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

## ✨ yew-router **0.12.0** *(2020-04-25)*

- #### 🚨 Breaking changes
  - Bump `yew` version to `0.15`.
- #### Extraneous
  - Remove `guide` example.

## ✨ yew **0.14.3** *(2020-04-04)*

- #### 🛠 Fixes

  - Remove `html!` component validation to allow generic components. [[@mankinskin], [#1065](https://github.com/yewstack/yew/pull/1065)]
  - Improve `Debug` formatting for `VTag` and `VText`. [[@dancespiele], [#1059](https://github.com/yewstack/yew/pull/1059)]
  - Implement `Default` for `Callback`. [[@TheNeikos], [#1043](https://github.com/yewstack/yew/pull/1043)]

## ✨ yew **0.14.2** *(2020-03-23)*

- #### 🛠 Fixes

  - Fix issue where components were rendered out of order. [[@mrh0057] & [@jstarry], [#1051](https://github.com/yewstack/yew/pull/1051)]
  - Reset Select component correctly in Firefox / Edge. [[@kuy], [#987](https://github.com/yewstack/yew/pull/987)]

## ✨ yew **0.14.1** *(2020-03-14)*

- #### 🛠 Fixes

  - `Connected` message was only called for first bridge creation. [[@nicklaswj], [#1029](https://github.com/yewstack/yew/pull/1029)]

## ✨ yew **0.14** *(2020-03-14)*

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

## ✨ yew-router **0.11.0** *(2020-03-14)*

- #### 🛠 Fixes
  - Fixed docs.rs document generation [[254](https://github.com/yewstack/yew_router/pull/254)] (Thanks @jetli)
  - Fixed clippy for web_sys target [[249](https://github.com/yewstack/yew_router/pull/249)] (Thanks @jetli)

## ✨ yew **0.13.2** *(2020-03-05)*

- #### 🛠 Fixes

  - Fix clippy warning when building with `web_sys` feature. [[@jstarry], [#1001](https://github.com/yewstack/yew/pull/1001)]

## ✨ yew **0.13.1** *(2020-03-04)*

- #### 🛠 Fixes

  - Fix for `web-sys` version `0.3.36`. [[@detegr], [#997](https://github.com/yewstack/yew/pull/997)]

## ✨ yew-router **0.10.0** *(2020-03-02)*

- Bumped version of Yew from v0.12.0 to v0.13.0
- This brings support for web_sys, which necessitates specifying either "web_sys" or "std_web" as a feature. (Thanks @tarkah)

## ✨ yew **0.13** *(2020-03-01)*

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

## ✨ yew-router **0.9.0** *(2020-02-25)*
- #### ⚡️ Features
  - Improved error handling in macro. [[233](https://github.com/yewstack/yew_router/pull/233)] @jplatte
- #### 🛠 Fixes
  - Fix RouterAnchor href [[228](https://github.com/yewstack/yew_router/pull/228)] @jetli
  - Undo non-passive state for prevent_default [[240](https://github.com/yewstack/yew_router/pull/240)] @jetli

## ✨ yew **0.12** *(2020-02-16)*

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

## ✨ yew-router **0.8.1** *(2020-01-10)*

- #### 🛠 Fixes
  - Fixed a dependency issue with `wasm-bindgen` that would cause builds to fail when building for the `wasm32-unknown-unknown` target.

## ✨ yew-router **0.8.0** *(2020-01-09)*
- #### ⚡️ Features
    - Use a default type parameter of `()` to specify state-related type parameters instead of the old macro-based solution. [[157](https://github.com/yewstack/yew_router/issues/157)]
    - Remove need for `JsSerializable` bound on the state parameter used for storing extra data in the history API.[[185](https://github.com/yewstack/yew_router/issues/185)]
    - RouterLink and RouterButton now support having children Html. This deprecates the `text` prop. [[192](https://github.com/yewstack/yew_router/issues/192)]
    - Fragment routing is now easily implementable by using an adapter because parser rules for the routing syntax were relaxed. [[195](https://github.com/yewstack/yew_router/issues/195)] [[211](https://github.com/yewstack/yew_router/pull/211)]
    - Support using this library only with the Switch derive, allowing it to run in non-web contexts. [[199](https://github.com/yewstack/yew_router/issues/199)]
- #### 🚨 Breaking changes
  - If you were using `default-features = false`,  you will have to now specify `features = ["service"]` to get the same behavior as before. [[199](https://github.com/yewstack/yew_router/issues/199)]
  - `RouterAnchor` and `RouterButton` now have props that take a `route: SW where SW: Switch` prop instead of a `link: String` and they now have a mandatory type parameter that specifies this `SW`. [[207](https://github.com/yewstack/yew_router/issues/207)]
  - `Route`'s state field now holds a `T` instead of an `Option<T>`. [[205](https://github.com/yewstack/yew_router/issues/205)]
  - Using default type parameters to specify the state typ instead of the macro that generated a module (`unit_state`) means that any imports from that module should now be replaced with the path that the type normally has in the project. [[157](https://github.com/yewstack/yew_router/issues/157)]
- #### Inconsequential
  - Change state related type parameters from `T` to `STATE`. [[208](https://github.com/yewstack/yew_router/issues/208)]

## ✨ yew **0.11** *(2020-01-06)*

This release aims to lay the groundwork for Yew component libraries and clean up the API for the ever elusive 1.0 release.

### Transition Guide

This release comes with a lot of breaking changes. We understand it's a hassle to update projects but the Yew team felt it was necessary to rip a few bandaids off now as we approach a 1.0 release in the (hopefully) near future. To ease the transition, here's a guide which outlines the main refactoring you will need to do for your project. (Note: all of the changes are reflected in the many example projects if you would like a proper reference example)

#### 1. Callback syntax

This is the main painful breaking change. It applies to both element listeners as well as `Component` callback properties. A good rule of thumb is that your components will now have to retain a `ComponentLink` to create callbacks on demand or initialize callbacks in your component's `create()` method.

Before:
```rust
struct MyComponent;

enum Msg {
    Click,
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MyComponent
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
impl Component for MyComponent {
    // ...

    fn view(&self) -> Html<Self> {
        html! { /* ... */ }
    }
}
```

After:
```rust
impl Component for MyComponent {
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
  struct MyComponent {
      onclick: Callback<ClickEvent>,
  }

  enum Msg {
      Click,
  }

  impl Component for MyComponent {
      type Message = Msg;
      type Properties = ();

      fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
          MyComponent {
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


## ✨ yew **0.10** *(2019-11-11)*

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
    impl Component for MyComponent {
        type Message = Msg;
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            MyComponent {}
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            true
        }
    }

    impl Renderable<MyComponent> for MyComponent {
        fn view(&self) -> Html<Self> {
            html! { "hello" }
        }
    }
    ```

    After:
    ```rust
    impl Component for MyComponent {
        type Message = Msg;
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            MyComponent {}
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

## ✨ yew-router **0.7.0** *(2019-11-11)*

- #### ⚡️ Features
  - Redirects that happen in the `Router` component actually change the url in the browser [[171](https://github.com/yewstack/yew_router/issues/171)]
  - Allow parsing (almost) any character after a `#` is encountered in matcher strings. 
  This enables this library to be used as a fragment router. [[150](https://github.com/yewstack/yew_router/issues/150)]
- #### 🛠 Fixes
  - Allow `!` to appear after `{...}` in matcher strings. [[148](https://github.com/yewstack/yew_router/issues/148)]
  - Matcher strings can now start with `&`. [[168](https://github.com/yewstack/yew_router/issues/168)] 
- #### 🚨 Breaking changes
  - Upgrade to Yew 0.10.0
  - Switch components now need to implement `Clone` in order to be used with the `Router` [[171](https://github.com/yewstack/yew_router/issues/171)]

## ✨ yew-router **0.6.1** *(2019-11-01)*
- #### ⚡️ Features
  - Bring back `{}`, `{*}`, and `{<number>}` capture syntax for tuple structs/enum variants. 
  If your variant or struct doesn't have named fields, you don't need to supply names in the matcher string [[116](https://github.com/yewstack/yew_router/issues/116)]
  - Allow ! special character in more places.
  - Greatly improve the quality of matcher string parsing errors. [[171](https://github.com/yewstack/yew_router/issues/149)]
  - Add `impl<SW: Switch, T> From<SW> for Route<T>`. Now Routes can be created from Switches easily.
  - Allow escaping {, }, and ! special characters by using `{{`, `}}`, and `!!` respectively.
  - Provide a correct error message when attempting to derive `Switch` for a Unit struct/variant with a capture group.

## ✨ yew-router **0.6.0** *(2019-10-24)*
- #### ⚡️ Features
  - `Switch` trait and Proc Macro enables extracting data from route strings.
  - `Router` component added.
  - `RouterLink` and `RouterButton` helper components added.
- #### 🚨 Breaking changes
  - Nearly everything. Most items were renamed.
  - Upgrade to Yew 0.9.0

## ✨ yew **0.9.2** *(2019-10-12)*

- #### 🛠 Fixes

  - Fix `yew-macro` dependency version

## ✨ yew **0.9.1** *(2019-10-12)*

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

## ✨ yew **0.9** *(2019-09-27)*

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

## ✨ yew **0.8** *(2019-08-10)*

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

## ✨ yew **0.7** *(2019-07-19)*

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

## ✨ yew **0.6** *(2019-02-20)*

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

## ✨ yew **0.5** *(2019-02-01)*

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

## ✨ yew **0.4** *(2018-06-01)*
## ✨ yew **0.3** *(2018-04-23)*
## ✨ yew **0.2** *(2018-01-08)*
## ✨ yew **0.1** *(2017-12-31)*

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
