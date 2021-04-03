---
title: Agents
description: Yew 的 Actor 系统
id: version-0.17.3-agents
original_id: agents
---

Agents 和 Angular 的 [Services](https://angular.io/guide/architecture-services) 相似（但没有依赖注入），给 Yew 提供了 [Actor 模型](https://en.wikipedia.org/wiki/Actor_model)。Agents 可以用于在组件之间路由消息，而与它们在组件层次结构中的位置无关，或者可以用于协调全局状态，或者可以用于从主 UI 线程上卸载计算密集型任务，或者在不同的标签页间通信（在未来）。

Agents 使用 [web-workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers) 同时运行来实现并发。

## 生命周期

![Agent lifecycle](https://user-images.githubusercontent.com/42674621/79125224-b6481d80-7d95-11ea-8e6a-ab9b52d1d8ac.png)

## Agents 的类型

### Reaches

- Context - Bridges 将生成或连接到 UI 线程上的 agent。这可用于在组件和其它 Agents 之间协调状态。当没有 Bridge 连接到该 Agent 时，Agent 将消失。

- Job - 在 UI 线程上为每个新的 Bridge 生成一个新的 Agent。这对于将与浏览器通信的共享但独立的行为移出组件是很有用的。（待验证）任务完成后，Agent 将消失。

- Public - 与 Context 相同，但运行在自己的 web worker 中。

- Private - 与 Job 相同，但运行在自己的 web worker 中。

- Global (WIP)

## Agent 和组件之间的通信

### Bridges

Bridges 不仅可以使 agent 和组件之间进行双向通信，还允许 agents 相互通信。

### Dispatchers

Dispatchers 用于组件和代理之间的单向通信，允许组件将消息发送到 agent 。

## 开销

 web worker （私有和公共）中的 agents 会在发送和接收的消息时产生序列化开销。由于他们使用 [二进制代码](https://github.com/servo/bincode) 与其他线程进行通信，因此成本比仅调用函数要高得多。所以除非计算成本超过消息传递成本，否则您应将逻辑包含在 UI 线程 agents（Job 或 Context）中。

## 进一步阅读：

-  [pub_sub](https://github.com/yewstack/yew/tree/master/examples/pub_sub) 示例展示了组件如何使用 agents 相互通信。
