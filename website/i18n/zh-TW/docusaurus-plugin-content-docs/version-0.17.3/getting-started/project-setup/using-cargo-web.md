# 使用 cargo-web

Cargo web 是一個 cargo 的指令之一，用來建置客戶端的網頁應用程式。使用它可以非常輕鬆的建置與開發網頁應用程式。他也是唯一一個支援 Emscripten targets 的工具。詳情請見[這裡](https://github.com/koute/cargo-web)。

## Install

```bash
cargo install cargo-web
```

## Build

```bash
cargo web build
```

## Run

```bash
cargo web start
```

## 支援目標平台

* `wasm32-unknown-unknown`
* `wasm32-unknown-emscripten`
* `asmjs-unknown-emscripten`

:::note
更多關於 `*-emscripten` targets 的資訊，請自行安裝 Emscripten SDK。
:::

