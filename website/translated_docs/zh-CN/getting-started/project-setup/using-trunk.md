---
title: 使用trunk
---

## 安装

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

## 用法

请查看[“构建示例应用程序”](../build-a-sample-app.md)，以获取有关如何使用Trunk构建Yew应用程序的简短指南。

您还可以通过查看我们的[示例](https://github.com/yewstack/yew/tree/master/examples)来查看实际情况，所有示例都是使用Trunk构建的。

Trunk基于`index.html`文件构建您的应用，并将其作为配置文件。与` wasm-pack `不同，此工具实际上是为构建应用而设计的。这意味着您不需要添加` cdylib `库，并且可以使用` main `函数作为入口点。

要构建一个简单的Yew应用程序，您只需要在项目的根目录下创建`index.html`：

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

Trunk CLI提供了一些有用的命令，但是在开发过程中， `trunk serve`无疑是最有用的命令。它为您运行本地服务器，并在检测到更改时自动重建应用程序。

当您准备发布应用程序时，只需运行`trunk build --release` 。

当前摘要仅介绍了Trunk的部分功能，请务必查看[README](https://github.com/thedodd/trunk) ！
