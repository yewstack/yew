# Dashboard Example

This example uses `FetchService` and `WebSocketService` to load external data.

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

## Improvements

- Handle errors by showing them to the user (at the very least they should be logged)
- Create a `cargo-make` task to run the example along with the WebSocket server

The example is called "dashboard" but it doesn't look or act like one.
It could be changed to be more like a dashboard.
