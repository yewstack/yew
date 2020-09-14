# Custom Components Example

A list of 1000 counters which can be increased individually.
Also, you can change the color of these buttons from red to blue by pressing another button 10 times...

## Concepts

This example demonstrates how an application can be structured into
components defined in separate modules, and how you can use callbacks to send
messages to a component higher up in the hierarchy.

## The components

[lib.rs](src/lib.rs) defines the root component, named `Model`. It constructs
`Barrier` and `Counter` elements, and passes them callbacks that send
messages back to `Model`. The `lib` module also brings the other components
into the project with `mod` statements at the top of the file.

[button.rs](src/button.rs) defines a `Button` component with an `onsignal`
property. When the button is clicked, it generates an internal `Clicked`
message, which is handled in the `update` function by calling
`self.onsignal.emit`.

[barrier.rs](src/barrier.rs) defines a `Barrier` which contains five
`Button`s with identical behaviour. The `Barrier` has an `onsignal` property,
which is emitted if any of its child `Button`s is clicked.

[counter.rs](src/counter.rs) defines a `Counter` component with several
attributes that can be set using props.

## Improvements

This example needs to be reworked or removed.
It doesn't do anything useful with the concept which is demonstrated by many other examples.
