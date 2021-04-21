---
title: 使用 cargo-web
id: version-0.17.3-using-cargo-web
original_id: using-cargo-web
---

Cargo web 是一个用来构建 web 客户端应用的 Cargo 子命令，它让构建和部署 web 应用变得非常的简单。它同样也是唯一一个支持生成 Emscripten 目标代码的工具链。点击[这里](https://github.com/koute/cargo-web)了解更多。

**安装**

```bash
cargo install cargo-web
```

## 构建

```bash
cargo web build
```

## 运行

```bash
cargo web start
```

## 支持生成的目标代码

- `wasm32-unknown-unknown`
- `wasm32-unknown-emscripten`
- `asmjs-unknown-emscripten`

:::note
对于 `*-emscripten` 的目标代码， 你需要安装 Emscripten SDK。
:::
