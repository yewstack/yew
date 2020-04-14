---
description: Yew's Actor System
---

# Agents

Agents are similar to Angular's [Services](https://angular.io/guide/architecture-services) \(but without dependency injection\), and provide a Yew with an [Actor Model](https://en.wikipedia.org/wiki/Actor_model). Agents can be used to route messages between components independently of where they sit in the component hierarchy, or they can be used to create a global state, and they can also be used to offload computationally expensive tasks from the main thread which renders the UI. There is also planned support for using agents to allow Yew applications to communicate accross tabs \(in the future\).

In order for agents to run concurrently, Yew uses [web-workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers).

## Lifecycle

![Agent lifecycle](https://user-images.githubusercontent.com/42674621/79125224-b6481d80-7d95-11ea-8e6a-ab9b52d1d8ac.png)

## Types of Agents

#### Reaches

* Job - Spawn a new agent on the UI thread for every new bridge. This is good for moving shared but independent behavior that communicates with the browser out of components. \(TODO verify\) When the task is done, the agent will disappear.
* Context - Bridges will spawn or connect to an agent on the UI thread. This can be used to coordinate with state between components or other agents. When no bridges are connected to this agent, the agent will disappear.
* Private - Same as Job, but runs on its own web worker. 
* Public - Same as Context, but runs on its own web worker.
* Global \(WIP\)

## Communication between Agents and Components


### Bridges

A bridge allows bi-directional communication between an agent and a component. Bridges also allow agents to communicate with one another.
 
### Dispatchers

A dispatcher allows uni-directional communication between a component and an agent. A bridge allows a component to send messages to an agent.

## Overhead

Agents communicate by serializing their messages using bincode\(???\). So there is a higher performance cost than just calling functions. Unless the cost of computation or the need to coordinate across arbitrary components will outweigh the cost of message passing, you should contain your logic to functions where possible.

## Further reading
* The [pub_sub](https://github.com/yewstack/yew/tree/master/examples/pub_sub) example shows how components can use agents to communicate with each other.

