---
description: Yew's Actor System
---

# Agents

Agents are similar to Angular's [Services](https://angular.io/guide/architecture-services) \(but without dependency injection\), and provide a Yew with an [Actor Model](https://en.wikipedia.org/wiki/Actor_model). Agents can be used to route messages between components independently of where they sit in the component hierarchy, or they can be used to coordinate global state, or they can be used to offload computationally expensive tasks off of the main UI-thread, or communicate between different tabs \(in the future\).

Agents that run concurrently use [web-workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers) to achieve that concurrency.

## Lifecycle

&lt;TODO&gt;

## Types of Agents

#### Reaches

* Job - Spawn a new agent on the UI thread for every new bridge. This is good for moving shared but independent behavior that communicates with the browser out of components. \(TODO verify\) When the task is done, the agent will disappear.
* Context - Bridges will spawn or connect to an agent on the UI thread. This can be used to coordinate with state between components or other agents. When no bridges are are connected to this agent, the agent will disappear.
* Private - Same as Job, but runs on its own web worker. 
* Public - Same as Context, but runs on its own web worker.
* Global \(WIP\)

## Agent Communication

### Bridges

Bridges will connect to an agent and allow two way communication.

### Dispatchers

Dispatchers are like bridges, but they can only send messages to agents.

## Overhead

Agents communicate by serializing their messages using bincode\(???\). So there is a higher performance cost than just calling functions. Unless the cost of computation or the need to coordinate across arbitrary components will outweigh the cost of message passing, you should contain your logic to functions where possible.





