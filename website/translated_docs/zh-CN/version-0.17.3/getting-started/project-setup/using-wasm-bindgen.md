---
title: 使用 wasm-bindgen
id: version-0.17.3-using-wasm-bindgen
original_id: using-wasm-bindgen
---

## 安装

```bash
cargo install wasm-bindgen-cli
```

## 构建

首先构建将生成Wasm文件的应用程序。假设您是根据[一个简单的应用](../build-a-sample-app.md)的章节内容构建的。那么输出文件的路径为`target/wasm32-unknown-unknown/debug/yew-app.wasm` 。如果您将 crate 命名为其他名称，则 Wasm 文件的名称将不是`yew-app.wasm` ，而是您在`Cargo.toml`文件中`package.name`设置的名称。

```bash
cargo build --target wasm32-unknown-unknown
```

然后，运行 wasm-bindgen 的 CLI 。这个命令将在 `--out-dir` 所指定的目录中生成一组文件，其中包含应用程序编译后的 WebAssembly 和一个 JavaScript 包装器，该包装器将加载 Wasm 二进制文件并运行它。这是必要的，因为浏览器目前不能直接加载WebAssembly文件，而是需要通过 JavaScript 脚本加载它们。在[一个简单的应用](../build-a-sample-app.md)示例中，我们希望在 `static` 文件夹中生成文件（为此，您需要将 `--out-dir static` 作为参数传递给 `wasm-bindgen`  ），而这两个文件会被命名为 `wasm.js` 和 `wasm_bg.wasm`  （您可以通过将 `--out-name wasm` 作为参数传递给 `wasm-bindgen`  ）。

```bash
wasm-bindgen --target web --out-dir static --out-name wasm target/wasm32-unknown-unknown/debug/appname.wasm --no-typescript
```

## 部署

选取你喜爱的服务器。这里我们使用一个简单的 Python 服务器来部署项目

```bash
python -m http.server 8000
```

## 支持生成的目标代码

- `wasm32-unknown-unknown`

## 进一步阅读：

- [<code>wasm-bindgen</code>文档](https://rustwasm.github.io/docs/wasm-bindgen/)
