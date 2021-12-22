const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "bundled-dist");

module.exports = {
  mode: "production",
  stats: "errors-warnings",
  entry: {
    index: "./index.js"
  },
  output: {
    path: dist,
    publicPath: "bundled-dist/",
    filename: "[name].js"
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: "--out-name index"
    })
  ]
};
