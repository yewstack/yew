---
description: Yew 的官方路由器
---

# 路由器

[https://crates.io/crates/yew-router](https://crates.io/crates/yew-router)

單頁應用程式（SPA）中的路由器，會依據 URL 來顯示不同的畫面。當連結被點擊後，路由器沒有預設要請求遠端的資源， 而是將 URL 設定導向應用程式中的有效路由。路由器會偵測 URL 被更改，然後決定要渲染什麼畫面。

## 核心元素

### Route

包含一個字串，這個字串是網域名後的那串文字，並且可以選擇要不要將狀態存入 history api。

### RouteService

與瀏覽器溝通，存取路由。

### RouteAgent

擁有 RouteService 並且協調與更新，從應用程式邏輯造成的，或是從瀏覽器事件中造成的，路由的改變。

### Switch

`Switch` trait 用於讓 Route 在實作的 `trait` 之間來回轉換。 

### Router

Router 元件會與 `RouteAgent` 溝通，並且自動解析從 agent 到 switch 的 Routes，Routes  會在 render 的 屬性中被揭露，這個屬性會決定 switch 的結果如何被轉換成 `Html`。

## 如何使用路由器

首先，你要建立一個代表你的應用程式所有狀態的型別。特別注意，這個型別可以是 enum、struct 都可以，而且你可以透過在裡面實作 `Switch` 來巢狀其他項目

然後你必須為你的型別 derive `Switch` 。對 enums 來說，每一個變數都必須宣告 `#[at = "/some/route"]`，或是如果你用 struct，那就要 struct 的外部宣告。

```rust
#[derive(Switch)]
enum AppRoute {
  #[at = "/login"]
  Login,
  #[at = "/register"]
  Register,
  #[at = "/delete_account"]
  Delete, 
  #[at = "/posts/{id}"]
  ViewPost(i32),
  #[at = "/posts/view"]
  ViewPosts,
  #[at = "/"]
  Home
}
```

特別注意，這個巨集會試著依序配對每個變數，所以如果有任何路由可能配對到兩著不同的 `to` 宣告，那會配對到第一個，而第二個就永遠不會被配對到。舉例來說，如果你定義以下的 `Switch` ，那路由將永遠只會配對到 `AppRoute::Home`。

```rust
#[derive(Switch)]
enum AppRoute {
  #[at = "/"]
  Home,
  #[at = "/login"]
  Login,
  #[at = "/register"]
  Register,
  #[at = "/delete_account"]
  Delete, 
  #[at = "/posts/{id}"]
  ViewPost(i32),
  #[at = "/posts/view"]
  ViewPosts,
}
```

你還可以拿到 url 中的參數，透過在`#[at = ""]` 中宣告 `{}`。`{}` 代表下一個分隔符號（"/"、"?"、"&"、"\#" ）之前， url 中的參數。`{*}` 表示取得直到後續字符匹配為止之間的變數，如果不存在任何字串，則它將匹配任何內容。 `{<number>}` 表示取得特定數量的的分隔符號之前的變數。（例如： `{2}` 會取得兩個分隔符號之前的變數。）

對於有命名欄位的 struct 與 enum，你必須給出變數的名字，像是： `{user_name}` 或是 `{*:age}`。

Switch trait 可以協助取得比起字串要更有結構的變數。你可以實作 `Switch`，這樣你就可以得到特定結構的變數，而他會是一個 `unsize`。但如果這個 URL 無法被轉換，就會被視為沒有匹配。

