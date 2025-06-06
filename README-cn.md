<div align="center">
  <a href="https://yew.rs/" target="_blank"><img src="https://yew.rs/img/logo.png" width="150" /></a>

  <h1>Yew</h1>

  <p>
    <strong>Rust / Wasm 客户端 Web 应用框架</strong>
  </p>

  <p>
    <a href="https://crates.io/crates/yew"><img alt="Crate Info" src="https://img.shields.io/crates/v/yew.svg"/></a>
    <a href="https://docs.rs/yew/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-yew-green"/></a>
    <a href="https://discord.gg/VQck8X4"><img alt="Discord Chat" src="https://img.shields.io/discord/701068342760570933"/></a>
    <a href="https://gitlocalize.com/repo/7052/whole_project?utm_source=badge"> <img src="https://gitlocalize.com/repo/7052/whole_project/badge.svg" /> </a>
    <a href="https://blog.rust-lang.org/2020/12/31/Rust-1.56.1.html"><img alt="Rustc Version 1.56.1+" src="https://img.shields.io/badge/rustc-1.56%2B-lightgrey.svg"/></a>
  </p>

  <h4>
    <a href="https://yew.rs/">文档（稳定版）</a>
    <span> | </span>
    <a href="https://yew.rs/docs/next/">文档（最新版）</a>
    <span> | </span>
    <a href="https://github.com/yewstack/yew/tree/master/examples">示例</a>
    <span> | </span>
    <a href="https://github.com/yewstack/yew/blob/master/CHANGELOG.md">更新日志</a>
    <span> | </span>
    <a href="https://yew.rs/docs/more/roadmap">路线图</a>
    <span> | </span>
    <a href="https://yew.rs/zh-Hans">简体中文文档</a>
    <span> | </span>
    <a href="https://yew.rs/zh-Hant">繁體中文文檔</a>
    <span> | </span>
    <a href="https://yew.rs/ja">ドキュメント</a>
  </h4>
</div>

## 关于

**Yew** 是一个现代化的 Rust 框架，用于使用 WebAssembly 创建多线程前端 Web 应用。

* 提供声明式宏来定义包含 Rust 表达式的交互式 HTML。有 React 中 JSX 使用经验的开发者在使用 Yew 时会感到非常熟悉。
* 通过最小化每次页面渲染的 DOM API 调用，并使后台 Web Worker 处理变得简单，从而实现高性能。
* 支持 JavaScript 互操作，允许开发者利用 NPM 包并与现有的 JavaScript 应用集成。

*注意：Yew 尚未发布 1.0 版本。请做好因 API 变更而进行大规模重构的准备。*

## 贡献

Yew 是一个社区项目，我们欢迎来自各种背景的开发者做出各种形式的贡献，无论大小。我们希望 Yew 社区成为一个有趣友好的地方，所以请查看我们的[行为准则](https://github.com/yewstack/yew/blob/master/CODE_OF_CONDUCT.md)，了解哪些行为是不被允许的。

#### 🤠 Yew 新手？

通过帮助我们改进[文档](https://github.com/yewstack/yew/tree/master/website/docs)来开始了解这个框架。提高测试覆盖率的 Pull Request 也非常受欢迎。

#### 😎 寻找灵感？

查看社区精选的与 Yew / WebAssembly 相关的精彩内容列表：[jetli/awesome-yew](https://github.com/jetli/awesome-yew)。

#### 🤔 对某些内容感到困惑？

随时加入我们的 [Discord 聊天室](https://discord.gg/VQck8X4)或开启一个[新的"问题"issue](https://github.com/yewstack/yew/issues/new/choose)，以获得贡献者的帮助。问题往往会导致框架人体工程学的改进、更好的文档，甚至是新功能的开发！

#### 🙂 准备深入代码？

在查看[贡献指南](https://github.com/yewstack/yew/blob/master/CONTRIBUTING.md)后，请查看["好的第一个问题"](https://github.com/yewstack/yew/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)（它们急需关注！）。一旦你找到一个你感兴趣的问题，请随时将其分配给自己，如果需要指导，请不要犹豫，这些问题的复杂性各不相同。

#### 😱 发现了一个 bug？

请[报告所有 bug！](https://github.com/yewstack/yew/issues/new/choose)如果开发者有兴趣并有时间，我们很乐意帮助他们修复他们发现的 bug。

#### 🤓 想帮助翻译？

翻译可以在 [Yew GitLocalize 仓库](https://gitlocalize.com/repo/7052)上提交。如果你有兴趣成为某种语言的官方审核员，请在 Discord 上联系我们。

## 贡献者

### 代码贡献者

这个项目的存在要感谢所有做出贡献的人。
<a href="https://github.com/yewstack/yew/graphs/contributors"><img src="https://opencollective.com/yew/contributors.svg?width=890&button=false" /></a>

### 财务贡献者

成为财务贡献者，帮助我们维持社区。[[贡献](https://opencollective.com/yew/contribute)]

#### 个人

<a href="https://opencollective.com/yew"><img src="https://opencollective.com/yew/individuals.svg?width=890"></a>

#### 组织

通过您的组织支持这个项目。您的徽标将显示在这里，并链接到您的网站。[[贡献](https://opencollective.com/yew/contribute)]

<a href="https://opencollective.com/yew/organization/0/website"><img src="https://opencollective.com/yew/organization/0/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/1/website"><img src="https://opencollective.com/yew/organization/1/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/2/website"><img src="https://opencollective.com/yew/organization/2/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/3/website"><img src="https://opencollective.com/yew/organization/3/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/4/website"><img src="https://opencollective.com/yew/organization/4/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/5/website"><img src="https://opencollective.com/yew/organization/5/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/6/website"><img src="https://opencollective.com/yew/organization/6/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/7/website"><img src="https://opencollective.com/yew/organization/7/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/8/website"><img src="https://opencollective.com/yew/organization/8/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/9/website"><img src="https://opencollective.com/yew/organization/9/avatar.svg"></a>