# Changelog

<!-- START TEMPLATE

## ✨ **VERSION** *(DATE)*

- #### ⚡️ Features
  - Sample
- #### 🛠 Fixes
  - Sample
- #### 🚨 Breaking changes
  - Sample

END TEMPLATE-->

## ✨ **0.3.0** *6/30/20*

- #### ⚡️ Features
  - Sample
- #### 🛠 Fixes
  - Sample
- #### 🚨 Breaking changes
  - `FetchAction::Success` has been renamed to `FetchAction::Fetched`
- #### Deprecations
  - module `effect`
  - module `pure`
  - macro `function_component`
  - struct `ptr::Lrc<T>`

## ✨ **v0.2.0** *11/18/19*
- #### ⚡️ Features
  - Add new `FetchRequest` trait, `fetch_resource()` function, and `FetchState` enum 
  to simplify making fetch requests using futures.
  - Add `Default` implementations to `Irc` and `Mrc`.
