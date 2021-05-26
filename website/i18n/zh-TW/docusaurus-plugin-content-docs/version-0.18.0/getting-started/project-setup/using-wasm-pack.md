# 使用 wasm-pack

這個工具由 Rust / Wasm Working  Group 開發，是建置 WebAssembly 應用程式中，社群最活躍的開發工具。他可以幫忙打包程式碼進 `npm` 的模組中，同時也有一個相應的 [Webpack plugin](https://github.com/wasm-tool/wasm-pack-plugin) 可以配合使用，並輕鬆跟已經存在的 JavaScript 應用程式整合。詳情請參考[這裡](https://rustwasm.github.io/docs/wasm-pack/introduction.html)。

:::note
注意，使用 `wasm-pack` 時，`Cargo.toml` 的 crate-type 必須是 `cdylib`。
:::

## Install

```bash
cargo install wasm-pack
```

## Build

這個指令會編譯你的程式碼，並將編譯好的 WebAssembly 與用於啟動專案的 JavaScript wrapper，在 .`/pkg` 的資料夾中製作成一個 bundle。

```bash
wasm-pack build
```

## Bundle

有關更多 Rollup 的資訊，請參見這個[指南](https://rollupjs.org/guide/en/#quick-start)。

```bash
rollup ./main.js --format iife --file ./pkg/bundle.js
```

## Serve

你可以使用任何你喜歡的伺服器服務。這裡我們只示範用用簡易的 python 伺服器服務，在 [http://\[::1\]:8000](http://[::1]:8000) 啟動我們的專案。

```bash
python -m http.server 8000
```

## 支援目標平台

* `wasm32-unknown-unknown`

