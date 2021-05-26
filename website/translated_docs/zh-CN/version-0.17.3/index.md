---
title: Introduction
---

## Yew 是什么？

**Yew** 是一个设计先进的 [Rust](https://www.rust-lang.org/) 框架，目的是使用 [WebAssembly](https://webassembly.org/) 来创建多线程的前端 web 应用。

- **基于组件的框架**，可以轻松的创建交互式 UI。拥有 [React](https://reactjs.org/) 或 [Elm](https://elm-lang.org/) 等框架经验的开发人员在使用 Yew 时会感到得心应手。
- **高性能** ，前端开发者可以轻易的将工作分流至后端来减少 DOM API 的调用，从而达到异常出色的性能。
- **支持与 JavaScript 交互** ，允许开发者使用 NPM 包，并与现有的 JavaScript 应用程序结合。

### 加入我们 😊

- 你可以在这里 [GitHub issues page](https://github.com/yewstack/yew/issues) 报告 Bugs 或者是提出新的想法。
- 我们欢迎 pull requests 。 如果你想要帮助我们，先参考下 [good first issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) 吧！
- 我们的 [Discord chat](https://discord.gg/VQck8X4) 非常的热闹，也是一个问问题和解决问题的好地方！

### 准备好开始了吗？

点击下面的链接，来学习并编写你的第一个 Yew 前端 App ， 并通过丰富的社区示例项目来学习。

[项目设置](getting-started/project-setup.md)

### 还没有完全信服？

Yew 项目基于划时代的新技术，非常适合那些希望开发未来基础项目的开发者。接下来是一些我们相信 Yew 这样的框架将为成为未来 Web 开发的主流。

#### 等等，为什么选用 WebAssembly?

WebAssembly *(Wasm)* 是一种可移植的底层语言，并且可以由 Rust 编译而来。它在浏览器中可以以原生速度运行，还同时支持和 JavaScript 交互。这些在所有的主流浏览器中都已经提供。希望了解更多关于 WebAssembly 是如何为前端应用提速的，可以查看官方[用例](https://webassembly.org/docs/use-cases/).

值得注意的是，Wasm（目前还）并不是提高 Web 应用性能的万金油（原文：A Silver Bullet）就目前来说，在 WebAssembly 中使用 DOM API 仍然比从 JavaScript 中调用要慢。但只是暂时性问题的，[WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) 计划将解决这个问题。如果你想要了解更多关于这方面的信息，可以查看 Mozilla 的这篇[佳作](https://hacks.mozilla.org/2019/08/webassembly-interface-types/) 。

#### 好的，那为什么选用 Rust 呢？

Rust 是一门运行速度超快，并且以他丰富的类型系统和可信赖的所有权模型而闻名的语言。尽管它的学习曲线非常的陡峭，但是带来的回报完全成正比！Rust 已经连续四年在 Stack Overflow 开发者调查报告中被评选为最受喜爱的编程语言：[2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted)，[2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted)，[2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) 和 [2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages)。

Rust 同样可以用它丰富的类型系统和可信赖的所有权模型来帮助开发者编写出更加安全的代码。和那些在 JavaScript 中难以定位的竞争条件 Bug 们说再见吧！ 事实上，通过 Rust ，大部分的 Bugs 都将在项目上线之前的编写阶段被编译器发现。同时不用担心，当你的应用出现错误的时候，你仍然可以在浏览器的调试控制台中获得你 Rust 代码的完整的错误栈追踪。

#### 同类的项目？

我们非常愿意和其他的类似项目交流想法，并且相信通过这种方式，我们可以互相扶持进步来发挥出这个新技术的潜力。如果你对 Yew 没有兴趣，你可能会喜欢这些项目。

- [Percy](https://github.com/chinedufn/percy) - *"A modular toolkit for building isomorphic web apps with Rust + WebAssembly"*
- [Seed](https://github.com/seed-rs/seed) - *"A Rust framework for creating web apps"*
