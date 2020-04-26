# Minimal Example

This example shows how to use this library with only the "service" feature turned on.
Without most of the features, you lack the `Router` component and `RouteAgent` and its associated bridges and dispatchers.
This means that you must use the `RouteService` to interface with the browser to handle route changes.

Removing the `Router` component means that you have to deal with the `RouteService` directly and propagate change route messages up to the component that contains the `RouteService`.

The unit type aliases part of the prelude are not included without any features. You may want to turn that back for actual use.