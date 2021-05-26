---
description: 父元件與子元件的溝通橋樑
---

# Properties

屬性讓子元件與父元件可以互相溝通。

## Derive macro

不要嘗試自己實作 `Properties`，而是用`#[derive(Properties)]`derive 他。

### 必填的欄位

預設所有在 `Properties` struct 裡的欄位都是必填的。當必填的欄位沒有值，而元件在 `html!` 巨集中又被建立，編譯器就會報錯。如果希望欄位是可選的，可以使用 `#[prop_or_default]` 來讓該欄位有預設值。如果希望欄位預設特定值，可以使用 `#[prop_or_else(value)]` ，裡面的 value 就會是這個欄位的預設值。舉例來說，希望預設值是 `true`可以在欄位宣告上面這樣寫 `#[prop_or_else(true)]`. 通常可選的屬性，會用 `Option` ，且預設值為`None`。

### PartialEq

如果可以，最好在你的屬性上面 derive `PartialEq` 。他可以避免畫面多餘的渲染，更細節的內容請參考，**優化與最佳實例**的區塊。

## 屬性的記憶體與速度的開銷

在 `Component::view`,裡，你可以拿到元件狀態的參考，且用他來建立 `Html` 。但是屬性是有所有權的。這代表著為了建立屬性，並且將他們傳遞給子元件，我們必須取得被 `view` 方法拿走的所有權。 當將參考傳給元件時，可以透過隱式的複製來做到得到所有權。

這意味著，每個元件，都有從父元件傳遞下來的獨有的狀態複本，且每當你重新渲染一次元件，被重新渲染的元件的所有的子元件的屬性就會被重新複製一次。

代表如果你要在屬性中傳遞_大量_的資料（大於 10 KB 的字串之類的），你可能需要考慮將你的子元件變成一個回傳 `Html` 的方法，讓父元件呼叫，以避免資料被複製。

如果你不需要改變傳下去的資料，你可以用 `Rc` 將資料包裝起來，這樣就會只複製參考的指針，而不是資料本身。

## 範例

```rust
use std::rc::Rc;
use yew::Properties;

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

impl Default for LinkColor {
    fn default() -> Self {
        // 除非有指定，否則預設是藍色
        LinkColor::Blue
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    /// 連結必須要有一個目標
    href: String,
    /// 如果連結文字很大，複製字串的參考可以減少記憶體的開銷
    /// 但除非效能已經成為嚴重的問題，否則通常不建議這麼做
    text: Rc<String>,
    /// 連結的顏色
    #[prop_or_default]
    color: LinkColor,
    /// 如果為 None，那 view 方法將不會指定 size
    #[prop_or_default]
    size: Option<u32>,
    /// 當沒有指定 active，預設為 true
    #[prop_or(true)]
    active: bool,
}
```

