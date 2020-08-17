---
description: Yew 的 Actor 系統
---

# Agents

Agents 類似於 Angular 的 [Services](https://angular.io/guide/architecture-services) （但沒有依賴注入）而且提供 Tew 一個 [Actor Model](https://en.wikipedia.org/wiki/Actor_model). Agents 可以用來作為兩個元件間的路由訊息，而且與他們在元件間的層級關係獨立出來，所以他也可以用來作為一個全域的狀態，甚至可以用來減輕用來渲染 UI 畫面的主執行緒的大量運算任務。 未來，我們還規劃要讓 agents 幫忙 Yew 專案可以跨頁籤溝通。

為了讓 agents 可以並行， Yew 使用了 [web-workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers)。

## 生命周期

![Agent &#x751F;&#x547D;&#x5468;&#x671F;](https://user-images.githubusercontent.com/42674621/79125224-b6481d80-7d95-11ea-8e6a-ab9b52d1d8ac.png)

## Agents 的型別

#### 範圍

* Job - 在 UI 執行緒上，為每一個 bridge，新增一個 agent。這對於將「共享但獨立的行為」移出元件很有用。（待驗證）當工作結束，agent 會消失。
* Context - Bridges 會建立並連接上 UI 執行緒上的 agent。這可以用來協調元件與其他 agent 之間的狀態。當沒有任何 bridge 連接上這個 agent，這個 agnet 就會消失。
* Private - 與 Job 相同，但是是在自己的 web worker 上執行的。
* Public - 與 Context 相同，但是是在自己的 web worker 上執行的。
* Global （編寫中）

## 在 Agents 與元件之間溝通

### Bridges

bridge 允許 agent 與元件進行雙向的溝通。bridge 也允許 agents 之間互相溝通。

### Dispatchers

dispatcher 允許元件與 agnet 進行單向的溝通。dispatcher 也允許元件向 agnet 發送訊息。 

## 開銷

Agents 透過使用 [bincode](https://github.com/servo/bincode) 序列化他們的訊息，來溝通。所以比起呼叫方法，他的效能花費比較高。除非計算的成本，或是跨元件計算的成本，比傳遞訊息的成本要高，否則 agnet 的方法儘量只有包含單純的邏輯運算。

## 延伸閱讀

* [pub\_sub](https://github.com/yewstack/yew/tree/master/examples/pub_sub) 範例顯示了如何在 agnets 之間溝通。

