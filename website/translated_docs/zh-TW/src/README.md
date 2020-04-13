# 簡介

## 什麼是 Yew？

**Yew** 是現代的 [Rust](https://www.rust-lang.org/) 框架，使用 [WebAssembly](https://webassembly.org/) 來開發多執行緒的網頁前端應用程式

* **基於元件的框架**，可以輕鬆開發互動的使用者介面 \(UI\)。有使用過 [React](https://reactjs.org/) 與 [Elm](https://elm-lang.org/) 的開發者在使用 Yew 的時候更容易上手。
* **高效能**，減少 DOM API 的呼叫次數，並幫助開發者輕鬆的將行程分流到背景的 web workers 中執行
* **與 JavaScript 互通**，允許開發者使用 NPM 的套件，並可以與現有的 JavaScript 應用程式整合

### 加入我們 😊

* 你可以在 [GitHub issues page](https://github.com/yewstack/yew/issues) 回報錯誤或一起討論新的功能
* 我們歡迎大家多發 PR \(pull request\)。 如果你有興趣與我們一起開發，歡迎先在 issues page 中尋找 [good first issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)
* 如果你還有任何問題，都可以在我們的 [Gitter chatroom](https://gitter.im/yewframework/Lobby) 中發問

![&#x6211;&#x5011;&#x793E;&#x7FA4;&#x6B63;&#x5728;&#x84EC;&#x52C3;&#x767C;&#x5C55;&#xFF01;](https://img.shields.io/github/stars/yewstack/yew?color=009A5B&label=Github%20stars)

### Ready to dive in?

Click the link below to learn how to build your first Yew app and learn from community example projects

{% page-ref page="getting-started/project-setup/" %}

### **Still not convinced?**

This project is built on cutting edge technology and is great for developers who like to develop the foundational projects of tomorrow. Here are some reasons why we believe that frameworks like Yew are the future of web development.

#### **Wait, why WebAssembly?**

WebAssembly _\(Wasm\)_ is a portable low-level language that Rust can compile into. It runs at native speeds in the browser and is interoperable with JavaScript and supported in all major browsers. For ideas on how to get the most out of WebAssembly for your app, check out this list of [Use Cases](https://webassembly.org/docs/use-cases/).

It should be noted that using Wasm is not \(yet\) a silver bullet for improving the performance of a web app. As of right now, using DOM APIs from WebAssembly is still slower than calling them directly from JavaScript. This is a temporary issue which the [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) proposal aims to resolve. If you would like to learn more, check out this [excellent article](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) from Mozilla.

#### Ok, but why Rust?

Rust is blazing fast and reliable with its rich type system and ownership model. It has a tough learning curve but is well worth the effort. Rust has been voted the most loved programming language in Stack Overflow's Developer Survey four years in a row: [2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted), [2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted), [2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) and [2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages).

Rust also helps developers write safer code with its rich type system and ownership model. Say goodbye to hard to track down race condition bugs in JavaScript! In fact, with Rust, most of your bugs will be caught by the compiler before your app even runs. And don't worry, when your app does run into an error, you can still get full stack-traces for your Rust code in the browser console.

#### Alternatives?

We love to share ideas with other projects and believe we can all help each other reach the full potential of this exciting new technology. If you're not into Yew, you may like the following projects \(listed alphabetically\)

* [Draco](https://github.com/utkarshkukreti/draco) - _"A Rust library for building client side web applications with Web Assembly"_
* [Percy](https://github.com/chinedufn/percy) - _"A modular toolkit for building isomorphic web apps with Rust + WebAssembly"_
* [Seed](https://github.com/seed-rs/seed) - _"A Rust framework for creating web apps"_
* [Smithy](https://github.com/rbalicki2/smithy) - _"A framework for building WebAssembly apps in Rust"_

