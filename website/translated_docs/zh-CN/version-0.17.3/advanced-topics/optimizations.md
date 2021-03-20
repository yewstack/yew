---
title: 性能优化与最佳实践
sidebar_label: 性能优化
description: 使您的应用更快
id: version-0.17.3-optimizations
original_id: optimizations
---

## neq_assign

当组件从它的父组件接收 props 时，`change` 方法将被调用。除了允许你更新组件的状态，还允许你返回一个布尔类型的值 `ShouldRender` 来指示组件是否应该响应 props 的更改而重新渲染自身。

重新渲染的开销很大，你应该尽量避免。一个通用的法则是，你只应该在 props 实际更改时重新渲染。以下代码块展示了此法则，如果 props 和先前的 props 不同，则返回 `true`：

```rust
use yew::ShouldRender;

#[derive(PartialEq)]
struct ExampleProps;

struct Example {
    props: ExampleProps,
};

impl Example {
    fn change(&mut self, props: ExampleProps) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
```

但是我们可以更进一步！对于任何实现了`PartialEq`的项，可以使用一个 trait 和一个 blanket implementation 将这六行样板代码减少到一行。<br>看看 [`yewtil` 是如何为他的 `NeqAssign` trait](https://docs.rs/yewtil/*/yewtil/trait.NeqAssign.html) 实现此功能的。

## 有效使用智能指针（smart pointers）

**注意：如果不确定本节中使用的某些术语，阅读 Rust Book[关于智能指针](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)的章节将有助于理解。**

为了避免在重新渲染时为了创建 props 而克隆大量数据，我们可以使用智能指针来只克隆指针。如果在 props 和子组件中使用对相关数据的引用，而不是在实际数据，则可以避免克隆任何数据，当需要修改子组件中的数据时，可以在该组件中使用`Rc::make_mut`来对要更改数据进行克隆和获取其可变引用。

`Component::change`可以更好地判断 props 更改时是否需要重新渲染组件。因为可以比较指针指向的地址（即计算机内存中存储数据的位置）而不是比较数据的值；如果两个指针的指向相同地址，则它们指向数据的值一定相同。需要注意的是，相反的情况下不一定是这样！对于基础数据而言，即使基础数据的值相同，但他们的指针地址也可能不同 —— 在这种情况下，您应该比较基础数据本身。

要进行这种比较，您需要使用`Rc::ptr_eq`而不是`PartialEq` （使用相等运算符`==`比较数据时会自动使用 PartialEq）。 Rust 文档有<a href="https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.ptr_eq" data-md-type="link">更详细的`Rc::ptr_eq`信息</a>。

对于不是 `Copy` 类型的数据，这种优化是最有效的。如果您能低成本、甚至毫无成本地拷贝数据，就没必要将那些数据放入智能指针中。而对于`Vec`、`HashMap` 和 `String` 这种可以包含大量数据的结构，此时使用智能指针能带来性能上的提升。

优化效果在值永远不会被子组件更新时达到最佳，如果值很少被父组件更新，那优化效果还会更好。这让`Rc<_>s`成为包装纯组件属性值的理想选择。

## 视图函数

出于代码可读性的原因，将 `html!` 各个部分的代码迁移到他们自己的函数中通常是有意义的，这样不仅可以避免深层嵌套的 HTML 中出现代码过多缩进的情况，还是一种好的设计模式 —— 特别是在构建可组合的应用程序时，因为这样就可以在其他地方调用这些函数，从而减少需要编写的代码量。

## 纯组件

纯组件是指不会修改它们自身状态的组件，它们仅展示内容和向普通可变组件传递消息。它们与视图函数不同之处在于他们可以在 `html!` 宏中使用组件语法（`<SomePureComponent />`）而不是表达式语法（`{some_view_function()}`），并且根据它们的实现，它们可以被记忆化（这意味着一个函数被调用过后它的结果会被“保存”，当这个函数再次被同样的参数调用时，它将返回第一次的结果，而不是重新计算。） - 使用前面提到的 `neq_assign` 逻辑来防止因为相同的 props 而重新渲染。

Yew 没有原生支持纯组件或者函数式组件，但是可以通过外部库获取它们。

## 函数式组件（又名钩子）

函数式组件仍在开发中！有一个[项目委员会](https://github.com/yewstack/yew/projects/3)详细记录了他们的状态。

## 当他们开发完成后，此部分将会更新

## 使用 Cargo Workspaces 进行编译速度优化

可以说，使用 Yew 的最大缺点是编译时间长。编译时间似乎与 `html!` 宏块中的代码量相关。对于较小的项目，这通常不是什么大问题，但是对于跨多个页面的 web 应用程序，通常可以将代码拆分为多个 crates 以最大程度地减少编译器要做的工作。

你应该尝试让主 crate 处理路由和页面选择，将所有公用的代码移动到另一个 crate，然后为每一个页面创建一个不同的 crate，其中每个页面可能是一个不同的组件，或者只是一个产生 `Html`的大函数。把程序里每个 crate 中的相同的代码提取到一个单独的 crate 中，这样就可以在整个项目中重复使用。在最好的情况下，你将从重新构建所有代码到只重新构建主 crate 和一个页面的 crate。在最糟糕的情况下，当你在“公共” crate 中编辑内容时，你将回到起点：编译所有依赖此公用 crate 的代码，这可能就是除此之外的所有代码。

如果你的主 crate 过于庞大，或者你想在深层嵌套的页面（例如，在另一个页面顶部渲染的页面）中快速迭代，则可以使用一个示例 crate 创建一个更简单的主页面实现并在之上渲染你正在开发的组件。

## 减少二进制文件的大小

- 优化 Rust 代码
    - `wee_alloc` （使用小型的分配器（allocator））
    - `cargo.toml` （定义 release profile）
- 使用`wasm-opt`优化 wasm 代码

**注意：更多有关减小二进制文件大小的信息，请参见 [《Rust Wasm Book》](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size) 。**

### wee_alloc

[wee_alloc](https://github.com/rustwasm/wee_alloc)是一个很小的分配器，它比 Rust 二进制文件中通常使用的分配器小得多。用此分配器替换默认分配器可以减小 Wasm 文件大小，但会导致程序速度变慢、内存开销变大。

与减少的文件大小相比，速度和内存的损耗是很小的。更小的文件体积意味着您的页面将加载得更快，因此通常情况下都建议您使用此分配器来替换默认的分配器，除非您的应用程序十分依赖分配器的表现。

```rust
// 将 `wee_alloc` 作为全局的 allocator。
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### Cargo.toml

在你的 `Cargo.toml` 的 `[profile.release]`部分中使用以下设置，可以将 release 版本的文件构建的更小。

```text
[profile.release]
# 二进制文件会包含更少的代码
panic = 'abort'
# 对所有代码库进行优化(优化更好，构建更慢)
codegen-units = 1
# 优化大小(更加激进)
opt-level = 'z'
# 优化大小
# opt-level = 's'
# 对整个程序进行链接时优化(link time optimization)
lto = true
```

### wasm-opt

此外，还可以尝试优化`wasm`代码的大小。

Rust Wasm Book 中有关于减小 Wasm 二进制文件大小的章节：[缩小 .wasm 大小](https://rustwasm.github.io/book/game-of-life/code-size.html)

- 使用`wasm-pack`（默认情况下会在 release 构建中优化`wasm`）
- 直接对`wasm`文件使用`wasm-opt`

```text
wasm-opt wasm_bg.wasm -Os -o wasm_bg_opt.wasm
```

#### 参考 yew/examples/ 中“minimum”示例的构建大小

注意： `wasm-pack`同时优化了 Rust 和 Wasm 的代码。 而示例中使用的`wasm-bindgen` 没有进行任何 Rust 大小优化。

使用的工具 | 大小
:-- | :--
wasm-bindgen | 158KB
wasm-bindgen + wasm-opt -Os | 116KB
wasm-pack | 99 KB

## 进一步阅读：

- [Rust Book关于智能指针的章节](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust Wasm Book 中有关减小二进制文件大小的内容](https://rustwasm.github.io/book/reference/code-size.html#optimizing-builds-for-code-size)
- [Rust 配置信息相关的文档](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [二进制项目](https://github.com/WebAssembly/binaryen)
