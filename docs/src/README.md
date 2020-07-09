# 介紹

## Yew 是什麼？

**Yew** 是一個現代化的 [Rust](https://www.rust-lang.org/) 框架，以 [WebAssembly](https://webassembly.org/) 建立多執行緒的前端應用程式。

* 它是一個以元件為基礎的框架以便於建立互動式的介面。具備 [React](https://reactjs.org/) 及 [Elm](https://elm-lang.org/) 的開發者在使用 Yew 進行開發時應該會感到賓至如歸。
* 透過最小化 DOM API 的呼叫及協助開發者輕易地卸載中的工作到背景的 web worker 使得它有**絕佳的效能表現**。
* 它支援與 **JavaScript 相互協作能力**（interoperability）讓開發者可以利用 NPM 套件及整合既有的 JavaScript 應用程式。

### 加入我們 😊

* 你可以在 [GitHub issues page](https://github.com/yewstack/yew/issues) 回報 bugs 及針對功能進行討論
* 我們 ❤️ pull requests 。如果你想要提供幫助，別忘了先閱讀 [good first issues](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) ！
* 我們 [Gitter 聊天室](https://gitter.im/yewframework/Lobby) 非常活躍且是一個提問的好地方

![Our community is thriving!](https://img.shields.io/github/stars/yewstack/yew?color=009A5B&label=Github%20stars)

### 準備好「潛」入了嗎？

點選下列的連結學習如何建立你的第一個 Yew 應用程式並且從社群的範例專案中進行學習

{% page-ref page="getting-started/project-setup/" %}

### **還在猶豫嗎？**

這個專案是建立在最新穎的科技之上，對於想要打造未來基礎專案的開發者而言是絕佳的選擇。以下是我們認為 Yew 會成為網路開發未來式的幾個理由。

#### **等等，為何選擇 WebAssembly ？**

WebAssembly _\(Wasm\)_ 作為 Rust 的編譯目標之一是一個可攜式的低階語言。它在瀏覽器中具有原生的執行速度且可以跟 JavaScript 相互協作及得到所有主要瀏覽器的支援。如果你想知道如何在你的應用程式中充分利用 WebAssembly ，不妨檢閱這個[使用情境](https://webassembly.org/docs/use-cases/)列表。

要提醒的是， Wasm （尚且）不是改善網路應用程式效能的萬靈丹。最少目前為止，在使用 DOM API 上 Wasm 仍然比不上直接以 JavaScript 呼叫。但這僅是暫時的問題， [WebAssembly Interface Types](https://github.com/WebAssembly/interface-types/blob/master/proposals/interface-types/Explainer.md) 提案正是以它作為解決的目標。如果你想瞭解更多有這個議題的資訊，別忘了檢閱這篇 Mozilla 的[超棒文章](https://hacks.mozilla.org/2019/08/webassembly-interface-types/)。

#### 好的，但為什麼是 Rust 呢？

Rust 的速度極快，同時豐富的型別系統和所有權模型讓它非常可靠。雖然它有著非常陡峭的學習曲線，但這些付出非常值得。Rust 也分別在 [2016](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted) 、 [2017](https://insights.stackoverflow.com/survey/2017#most-loved-dreaded-and-wanted) 、 [2018](https://insights.stackoverflow.com/survey/2018#technology-_-most-loved-dreaded-and-wanted-languages) 、 [2019](https://insights.stackoverflow.com/survey/2019#technology-_-most-loved-dreaded-and-wanted-languages) 連續四年在 Stack Overflow 的開發者調查中獲選為最愛的程式設計語言。

Rust 同時透過豐富的型別系統及所有權模型協助開發者撰寫更加安全的程式碼。忘了 JavaScript 中難以追蹤的競賽條件吧！事實上，多數情況下 Rust 的編譯器甚至可以在你的應用程式執行前就把 bug 抓出來。而且就算你的應用程式真的在執行時發生了錯誤，你仍然可以在瀏覽器控制台中取得 Rust 程式碼的完整堆疊追蹤。

#### 替代選項？

我們喜愛跟其他專案分享點子而且相信在互相幫助之下可以激發這個最新科技的全部潛能。如果你不是 Yew 的一份子，那你或許會想要追蹤這些專案（以字母排序）

* [Draco](https://github.com/utkarshkukreti/draco) - _"一個結合 Web Assembly 並用於建立用戶端網路應用程式的 Rust 函式庫"_
* [Percy](https://github.com/chinedufn/percy) - _"一個 Rust + WebAssembly 的模組化工具組，用於建立同購的網路應用程式"_
* [Seed](https://github.com/seed-rs/seed) - _"一個建立網路應用程式的 Rust 框架"_
* [Smithy](https://github.com/rbalicki2/smithy) - _"一個以建立 WebAssembly 應用程式的 Rust 框架"_

