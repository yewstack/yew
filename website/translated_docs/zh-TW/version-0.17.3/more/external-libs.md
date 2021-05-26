---
description: 函式庫可以幫助 yew 的開發者
---

# 額外的函式庫

### Yewtil

Yewtil 是一個常見的工具懶人包，可以幫助你編寫 Yew 的程式碼，裡面包含了：

* NeqAssign - 如前面的章節所述，這是一個減少因為屬性改變而重新渲染的最佳方法。
* PureComponents - 不會更新自己狀態的元件。在他的生命周期底下使用 NeqAssign，可以讓他就像是 memoized 的函式。他可以寫在 `html!` 巨集中看起來就像是一般的元件。
* Lrc - linked list 參考計數的智慧指針函式，跟 `Rc` 差不多，但是他可以讓新的資料更新他的模型。
* Mrc/Irc - 可變與不可變的參考計數智慧指針，跟 Rc 差不多。但是因為 `Mrc` 實作了 `DerefMut` 與 `BorrowMut`，所以讓他在 Yew 中更便於使用。 這讓 `Mrc` 可以與 `NeqAssign` 一起使用。 `Irc` 在資料裡就像是不可變得 view，他可以管理只用來顯示用的資料。
* History - 一個歷史追蹤器，他用 `VecDeque` 管理先前的值。
* Futures - 支援運行 futures，他會送訊息給元件的更新迴圈。
* Fetch - 處理使用 web\_sys 與前面所提的 futures 的功能所發出的請求，的抽象層。

## 尋找

我們需要函式庫的社群生態，目前還沒有。

Boostrap/MaterialUI/任何 css 框架的元件封裝。

