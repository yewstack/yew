# Yew Router Examples
- minimal - Demonstrates how to use this library without the use of the Router component.
- router_component - Shows off the preferred way for how to use this library.

- switch - Various examples for how to construct routes with the router.

## Running

To run the examples you'll need to spin up a web server; two possible ways of doing this are either using a Rust webserver or the built-in Python webserver (we suggest this because Python is installed by default on most systems).

### Running with Rust (warp / actix)
Details on how to build and run these examples can be found in the `README.md` under `servers/`.

Using the router in its expected use case (not fragment routing) requires that the server respond to requests for
resources at URLs that are routes within the router with the index.html of the application.

### Running with Python
In applicable directories, launch the `run.sh` shell script.
