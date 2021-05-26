---
description: ComponentLink 與 Callbacks.
---

# Callbacks

元件的「link」是一個讓元件註冊 callbacks 並自我更新的機制。

## ComponentLink API

### callback

註冊一個 callback 後，當這個 callback 被執行時，會發送一個訊息給元件的更新機制。在生命周期的勾子下，他會呼叫 `send_self` 並將被閉包回傳的訊息帶給他。

提供一個 `Fn(IN) -> Vec<COMP::Message>` 並回傳一個 `Callback<IN>` 。

### send\_message

當現在的迴圈結束後，向元件發送訊息，並且開啟另一個迴圈。

### send\_message\_batch

註冊一個 callback，當這個 callback 被執行時，這個 callback 會一次送很多訊息。如果有任何一個訊息導致元件被重新渲染，元件會在所有批次送來的訊息都被處理完後，再重新渲染。

提供一個 `Fn(IN) -> COMP::Message` 並回傳一個 `Callback<IN>` 。

## Callbacks

_（他可能需要一個獨立的短頁來介紹）_

Callbacks 被用來當作 services 、 agents 與父元件跟 Yew 溝通的方式。他們只是一個被 `Rc` 包裹著的 `Fn`，好讓他們可以被複製。

他們有一個 `emit` 方法，這個方法拿他們的 `<IN>` 型別當作參數，並且轉換他作為目的地所期望的訊息。如果一個從父元件來的 callback被提供作為子元件的屬性，子元件可以在他的 update 生命周期中，呼叫 callback 中的 emit 以傳遞訊息回給父元件。 在 `html!` 巨集中的閉包與方法如果被當作屬性傳遞，會被自動轉為 Callbacks。

