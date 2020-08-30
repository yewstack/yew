# Dashboard example

This example uses the `fetch` and `websocket` services to load external data.

## Special requirements for WebSocket

This is not a requirement for the example as a whole, only for the parts relating to the WebSocket service.

The frontend assumes there's a WebSocket server listening at `ws://localhost:9001/`.
The [`server`](server) directory contains a very basic WebSocket server that can be used for this purpose.

Run the following commands to start it:

```bash
# assuming you're in the same directory as this README file.
cd server

cargo run
```

The server just echoes all the data it receives back to the client and logs the message to the console.
