---
title: 簡介
---

## 什麼是 Yew？

**Yew** 是現代化的 [Rust](https://www.rust-lang.org/) 框架，使用 [WebAssembly](https://webassembly.org/) 來開發多執行緒的網頁前端應用程式

* **基於元件的框架**，可以輕鬆開發互動的使用者介面 \(UI\)。有使用過 [React](https://reactjs.org/) 與 [Elm](https://elm-lang.org/) 的開發者在使用 Yew 的時候更容易上手。
* **高效能**，減少 DOM API 的呼叫次數，並幫助開發者輕鬆的將行程分流到背景的 web workers 中執行
* **與 JavaScript 互通**，允許開發者使用 NPM 的套件，並可以與現有的 JavaScript 應用程式整合

### 加入我們 😊

* 你可以在 [GitHub issues page](https://github.com/yewstack/yew/issues) 回報 bugs 及針對功能進行討論
* 我們歡迎大家多發 PR \(pull request\)。 如果你有興趣與我們一起開發，別忘了先閱讀 [good first issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) ！
* 我們 [Gitter](https://gitter.im/yewframework/Lobby) 聊天室非常活躍，歡迎提問

### 準備好「潛」入了嗎？

點選下列的連結學習如何建立一個 Yew 應用程式並且從社群的範例專案中學習

[專案設定](getting-started/project-setup.md)

### 還在猶豫嗎？

這個專案是建立在最新穎的科技之上，對於想要打造未來基礎專案的開發者而言是絕佳的選擇。以下是我們認為 Yew 會成為網路開發未來式的幾個理由。

#### 等等，為何選擇 WebAssembly ？

WebAssembly \(Wasm\) 是一個可攜式的低階語言也是 Rust 的編譯目標之一。它在瀏覽器中具有原生的執行速度且可以跟 JavaScript 相互協作還得到所有主要瀏覽器的支援。如果你想知道如何在應用程式中充分利用 WebAssembly ，不妨檢閱這個[使用情境](https://webassembly.org/docs/use-cases/)列表。 

注意， Wasm （尚且）不是改善網路應用程式效能的萬靈丹。最少目前為止，在使用 DOM API 上 Wasm 仍然比不上直接以 JavaScript 呼叫。但這僅是暫時的問題， [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) 提案正是以它作為解決的目標。如果你想瞭解更多有這個議題的資訊，別忘了檢閱這篇 Mozilla 的[超棒文章](https://hacks.mozilla.org/2019/08/webassembly-interface-types/)。

#### 好的，但為什麼是 Rust 呢？

Rust 的速度極快，同時豐富的型別系統和所有權模型也讓它非常可靠。雖然它的學習曲線陡峭，但這些付出會非常值得。Rust 也分別在 [2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted) 、 [2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted) 、 [2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) 、 [2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) 連續四年在 Stack Overflow 的開發者調查中獲選為最喜愛的程式設計語言。

Rust 豐富的型別系統及所有權模型協助開發者撰寫更加安全的程式碼。忘了 JavaScript 中難以追蹤的競賽條件吧！在多數情況下 Rust 的編譯器甚至可以在你的應用程式執行前就把 bug 抓出來。而且就算你的應用程式真的在執行時發生了錯誤，你仍然可以在瀏覽器控制台中取得 Rust 程式碼的完整堆疊追蹤。

#### 有其他選擇嗎？

我們喜愛跟其他專案分享點子而且相信在互相幫助之下可以激發這個最新科技的全部潛能。如果你不是 Yew 的一份子，那你或許會想要關注這些專案。

* [Percy](https://github.com/chinedufn/percy) —_「 Rust + WebAssembly 的模組化工具組，用於建立同購的網路應用程式」_
* [Seed](https://github.com/seed-rs/seed) —_「建立網路應用程式的 Rust 框架」_
