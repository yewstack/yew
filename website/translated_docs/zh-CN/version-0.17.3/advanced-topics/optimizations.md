---
description: 加速你的应用程序
---

# 性能优化与最佳实践

## neq\_assign

当组件从它的父组件接收 props 时，`change` 方法将被调用。除了允许你更新组件的状态，还允许你返回一个布尔类型的值 `ShouldRender` 来指示组件是否应该响应 props 的更改而重新渲染自身。

重新渲染的开销很大，你应该尽量避免。一个通用的法则是，你只应该在 props 实际更改时重新渲染。以下代码块展示了此法则，如果 props 和先前的 props 不同，则返回 `true`：

```rust
fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.props != &props {
        *self.props = props;
        true
    } else {
        false
    }
}
```

但是我们可以更进一步！对于任何实现了 `PartialEq` 的项，可以使用一个 trait 和一个 blanket implementation 将这六行样板代码减少到一行。

{% code title="neq\_assign.rs" %}
```rust
pub trait NeqAssign {
    fn neq_assign(&mut self, new: Self) -> ShouldRender;
}
impl<T: PartialEq> NeqAssign for T {
    fn neq_assign(&mut self, new: T) -> ShouldRender {
        if self != &new {
            *self = new;
            true
        } else {
            false
        }
    }
}

// ...
fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props.neq_assign(props)
}
```
{% endcode %}

该 trait 称为 `NeqAssign` 是因为如果目标值和新值不相等，它将赋为新值。

这比简单的实现还要短：

```rust
// 不要这样做，除非你无法避免。
fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
}
```

你不仅限在 `change` 函数中使用它。通常，在 `update` 函数中执行此操作也是有意义的，尽管性能提升在那里不太明显。

## wee\_alloc

[wee\_alloc](https://github.com/rustwasm/wee_alloc) 是一个比 Rust 二进制文件中通常使用的分配器还小得多的微型分配器。用这个分配器来替代默认的分配器将使 WASM 文件体积更小，但会牺牲速度和内存开销。

对比不包含默认分配器换取的体积大小，牺牲的速度和内存开销是微不足道的。较小的文件体积意味着你的页面将加载更快，因此通常建议使用此分配器而不是默认分配器，除非你的应用程序会执行一些繁重的内存分配任务。

```rust
// 将 `wee_alloc` 作为全局分配器
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

## RC

为了避免在重新渲染时为了创建 props 而克隆大块数据，我们可以使用智能指针来只克隆指针。如果在 props 和子组件中使用 `Rc<_>` 而不是普通未装箱的值，则可以延迟克隆直到需要修改子组件中的数据为止，在该组件中可以使用 `Rc::make_mut` 来对要更改数据进行克隆和获取可变引用。通过在要修改前不进行克隆，子组件可以在几乎没有性能成本的情况下拒绝与它们在 `Component::change` 中拥有状态的 props 相同的 props，这与数据本身需要先复制到父级 props 结构体中，然后在子级中进行比较和拒绝的情况相反。

对于不是 `Copy` 类型的数据，这种优化是最有用的。如果你能轻松地拷贝数据，那么将其放入智能指针中可能是不值得的。对于可以包含大量数据的结构，例如 `Vec`，`HashMap` 和 `String`，这种优化应该是值得的。

如果子组件从不更新组件的值，则这种优化效果最好，如果父组件很少更新组件的值，则效果更好。这使得 `Rc<_>s` 是包装纯组件属性值的不错选择。

## 视图函数

出于代码可读性的原因，将 `html!` 各个部分的代码迁移到他们自己的函数中通常是有意义的，这样就可以避免在深层嵌套的 HTML 中出现代码块向右偏移。

## 纯组件 / 函数式组件

纯组件是不会修改它们状态的组件，它们仅展示内容和向普通可变组件传递消息。它们与视图函数不同之处在于他们可以使用组件语法（`<SomePureComponent />`）而不是表达式语法（`{some_view_function()}`）来在 `html!` 宏中使用，并且根据它们的实现，它们可以被记忆化 - 使用前面提到的 `neq_assign` 逻辑来防止因为相同的 props 而重新渲染。

Yew 没有原生支持纯组件或者函数式组件，但是可以通过外部库获取它们。

函数式组件尚不存在，但是从理论上来讲，可以通过使用 proc 宏和标注函数生成纯组件。

## Keyed DOM nodes when they arrive

## 使用 Cargo Workspaces 进行编译速度优化

可以说，使用 Yew 的最大缺点是编译时间长。编译时间似乎与 `html!` 宏块中的代码量相关。对于较小的项目，这通常不是什么大问题，但是对于跨多个页面的 web 应用程序，将代码拆分为多个 crates 以最大程度地减少编译器要做的工作通常是有意义的。

你应该尝试让主 crate 处理路由和页面选择，将所有公用的代码移动到另一个 crate，然后为每一个页面创建一个不同的 crate，其中每个页面可能是一个不同的组件，或者只是一个产生 `Html` 的大函数。在最好的情况下，你将从重新构建所有代码到只重新构建主 crate 和一个页面的 crate。在最糟糕的情况下，当你在“公共” crate 中编辑内容时，你将回到起点：编译所有依赖此公用 crate 的代码，这可能就是除此之外的所有代码。

如果你的主 crate 过于庞大，或者你想在深层嵌套的页面（例如，在另一个页面顶部渲染的页面）中快速迭代，则可以使用一个示例 crate 创建一个更简单的主页面实现并在之上渲染你正在开发的组件。

