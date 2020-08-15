# Changelog

<!-- START TEMPLATE

## ✨ **VERSION** *(DATE)*

- #### ⚡️ Features
  - None 
- #### 🛠 Fixes
  - None 
- #### 🚨 Breaking changes
  - None 

END TEMPLATE-->


## ✨ **0.15.0** *(TBD)*

- #### ⚡️ Features
  - None
- #### 🛠 Fixes
  - None
- #### 🚨 Breaking changes
  - `RouterButton` now prevents default events per default @TheNeikos

## ✨ **0.14.0** *2020-6-30*

- #### ⚡️ Features
  - None 
- #### 🛠 Fixes
  - None 
- #### 🚨 Breaking changes
  - The `unit_state` module has been removed. 
  - Bump `yew` version to `0.17`.


## ✨ **0.13.0** *2020-5-12*

- #### 🚨 Breaking changes
  - Bump `yew` version to `0.16`.

## ✨ **0.12.1** *2020-4-26*
- #### 🛠 Fixes
  - Fix infinite rerender bug in 'Router' component. (Thanks @dancespiele)

## ✨ **0.12.0** *2020-4-25*

- #### 🚨 Breaking changes
  - Bump `yew` version to `0.15`.
- #### Extraneous
  - Remove `guide` example.

## ✨ **0.11.0** *2020-3-14*

- #### 🛠 Fixes
  - Fixed docs.rs document generation [[254](https://github.com/yewstack/yew_router/pull/254)] (Thanks @jetli)
  - Fixed clippy for web_sys target [[249](https://github.com/yewstack/yew_router/pull/249)] (Thanks @jetli)
  
  
## ✨ **0.10.0** *2020-3-2*

- Bumped version of Yew from v0.12.0 to v0.13.0
- This brings support for web_sys, which necessitates specifying either "web_sys" or "std_web" as a feature. (Thanks @tarkah)

## ✨ **0.9.0** *2020-2-25*
- #### ⚡️ Features
  - Improved error handling in macro. [[233](https://github.com/yewstack/yew_router/pull/233)] @jplatte
- #### 🛠 Fixes
  - Fix RouterAnchor href [[228](https://github.com/yewstack/yew_router/pull/228)] @jetli
  - Undo non-passive state for prevent_default [[240](https://github.com/yewstack/yew_router/pull/240)] @jetli
  
  
## ✨ **0.8.1** *(2020-1-10)*

- #### 🛠 Fixes
  - Fixed a dependency issue with `wasm-bindgen` that would cause builds to fail when building for the `wasm32-unknown-unknown` target.

## ✨ **0.8.0** *(2020-1-9)*
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
  
## ✨ **0.7.0** *(2019-11-11)*

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

## ✨ **0.6.1** *(2019-11-1)*
- #### ⚡️ Features
  - Bring back `{}`, `{*}`, and `{<number>}` capture syntax for tuple structs/enum variants. 
  If your variant or struct doesn't have named fields, you don't need to supply names in the matcher string [[116](https://github.com/yewstack/yew_router/issues/116)]
  - Allow ! special character in more places.
  - Greatly improve the quality of matcher string parsing errors. [[171](https://github.com/yewstack/yew_router/issues/149)]
  - Add `impl<SW: Switch, T> From<SW> for Route<T>`. Now Routes can be created from Switches easily.
  - Allow escaping {, }, and ! special characters by using `{{`, `}}`, and `!!` respectively.
  - Provide a correct error message when attempting to derive `Switch` for a Unit struct/variant with a capture group.

## ✨ **0.6.0** *(2019-10-24)*
- #### ⚡️ Features
  - `Switch` trait and Proc Macro enables extracting data from route strings.
  - `Router` component added.
  - `RouterLink` and `RouterButton` helper components added.
- #### 🚨 Breaking changes
  - Nearly everything. Most items were renamed.
  - Upgrade to Yew 0.9.0
