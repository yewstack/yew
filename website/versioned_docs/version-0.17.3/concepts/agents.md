---
title: Agents
description: Yew's Actor System
id: version-0.17.3-agents
original_id: agents
---

Agents are similar to Angular's [Services](https://angular.io/guide/architecture-services) \(but without dependency injection\), and provide a Yew with an [Actor Model](https://en.wikipedia.org/wiki/Actor_model). Agents can be used to route messages between components independently of where they sit in the component hierarchy, or they can be used to create a shared state, and they can also be used to offload computationally expensive tasks from the main thread which renders the UI. There is also planned support for using agents to allow Yew applications to communicate across tabs \(in the future\).

In order for agents to run concurrently, Yew uses [web-workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers).

## Lifecycle

![Agent lifecycle](https://user-images.githubusercontent.com/42674621/79125224-b6481d80-7d95-11ea-8e6a-ab9b52d1d8ac.png)

## Types of Agents

### Reaches

* Context - There will exist at most one instance of a Context Agent at any given time. Bridges will
  spawn or connect to an already spawned agent on the UI thread. This can be used to coordinate
  state between components or other agents. When no bridges are connected to this agent, the agent
  will disappear.

* Job - Spawn a new agent on the UI thread for every new bridge. This is good for moving shared but
  independent behavior that communicates with the browser out of components. \(TODO verify\) When
  the task is done, the agent will disappear.

* Public - Same as Context, but runs on its own web worker.

* Private - Same as Job, but runs on its own web worker.

* Global \(WIP\)

## Communication between Agents and Components

### Bridges

A bridge allows bi-directional communication between an agent and a component. Bridges also allow agents to communicate with one another.

### Dispatchers

A dispatcher allows uni-directional communication between a component and an agent. A dispatcher allows a component to send messages to an agent.

## Overhead

Agents that live in their own separate web worker \(Private and Public\) incur serialization overhead on the messages they send and receive. They use [bincode](https://github.com/servo/bincode) to communicate with other threads, so the cost is substantially higher than just calling a function. Unless the cost of computation will outweigh the cost of message passing, you should contain your logic in the UI thread agents \(Job or Context\).

## Further reading

* The [pub\_sub](https://github.com/yewstack/yew/tree/master/examples/pub_sub) example shows how components can use agents to communicate with each other.
