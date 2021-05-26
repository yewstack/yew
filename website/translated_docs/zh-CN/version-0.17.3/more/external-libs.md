---
description: 可以帮助 Yew 开发的库
---

# 外部库

### Yewtil

Yewtil 是一个帮助你编写 Yew 程序的通用工具集合。它包括：

* NeqAssign - 如前所述，这是给 props 赋值以保证最小化重新渲染的最佳方式。
* PureComponents - 不更新任何状态的组件。它们在内部使用 NeqAssign，表现得像记忆化函数并像普通组件一样在 `html!` 宏内部被调用。
* Lrc - 链表引用计数智能指针，功能类似于 `Rc`，但是允许新颖的数据更新模式。
* Mrc/Irc - 可变/不可变引用计数智能指针，功能类似于 `Rc`，但由于为 `Mrc` 实现了 `DerefMut` 和 `BorrowMut`，因此在 Yew 中对用户更友好。这允许 `Mrc` 与 `NeqAssign` 一起使用。`Irc` 充当数据的不可变视图，这使其成为保存仅用于显示的数据的理想选择。
* History - 历史记录追踪包装器，它使用 `VecDeque` 来保存其展示过的历史值。
* Futures - 支持运行将消息发送到组件更新循环的 futures。
* Fetch - 用于处理使用 `web-sys` 和前面提到的 Futures 功能发请求的抽象。

## 寻求

生态需要但目前还没有的库。

Bootstrap/MaterialUi/任意 css 框架的组件封装。

