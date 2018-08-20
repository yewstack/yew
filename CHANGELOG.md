# Changelog

## 0.5 - unreleased

### Breaking changes

- Context requirement removed. Not necessary to use `Component<CTX>` type parameter.
  Instead of a context a link to an environment provided with `Component::create` call.
  All examples had changed.

- `html!` macro adds `move` modifier and the type of event for every handler (#240). Use
`<input oninput=|e| Msg::UpdateEvent(e.value), />` instead of obsolete
`<input oninput=move |e: InputData| Msg::UpdateEvent(e.value), />`.

### New features

- Added `Agent`s concept. Agents are separate activities which you could run in the same thread
  or in a separate thread. There is `Context` kind of agent that spawn context entities as many
  as you want and you have to interact with a context by a messages. To join an agent use
  `Worker::bridge` method and pass a link of component's environment to it.

- Added three types of agents: `Context` - spawns once per thread, `Job` - spawns for every bridge,
  `Public` - spawns an agent in a separate thread (it uses [Web Workers API] under the hood).

- Added `<Component: with props />` rule to set a whole struct as a properties of a component.

- All services are reexported within the `yew::services` module.

- `html!` macro supports multiple classes in a single string. For example:
  `<a class="button is-primary",>{ "Send" }</a>`.

- Added `FetchOptions` to set `Credentials` of `fetch` request.

- `FetchService` aborts requests using `AbortController`.

- Added `SubmitEvent` with `onsubmit` rule.

- Added `send_self` method to `ComponentLink` to send messages itself immediately.

[Web Workers API]: https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API

### Bug fixes

- Bug with emscripten target `RuntimeError: index out of bounds` (#220) fixed with a new scheduler.
