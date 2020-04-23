# Render

The `render` prop in a `Route` takes a `Render` struct, which is just a wrapper around a `Fn(&Matches) -> Option<Html<Router>>`.
The reason it returns an `Option` is that it allows for rejection of routes that have captured sections that can't meet restrictions that be easily expressed in a matcher string.

### Example 
```rust

html! {
    <Router>
        <Route path=route!("/a/{capture}") render=render(|matches: &Matches| {
            let number: usize = matches["capture"].parse().ok()?;
            Some(
                html! {
                    format!("Only positive numbers allowed here: {}", number)
                }
            ) 
        }) />
    </Router>
}
```

## `render` function
The `render` function takes a function that implements `Fn(&Matches) -> Option<Html<Router>>`, and all it does is wrap the provided function in a `Render` struct.

### Example
```rust
let r: Render<()> = render(|_matches: &Matches| Some(html!{"Hello"}));
```

## `component` function
The `component` function is a way to create this `Render` wrapper by providing the type of the component you want to render as a type parameter.

The only caveat to being able to use the `component` function, is that the `Properties` of the specified component must implement the `FromCaptures` trait. 
`FromCaptures` mandates that you implement a function called `from_matches` which has a type signature of, `Fn(&Matches) -> Option<Self>`.
Code in `component` takes the props created from this function and creates a component using them.

There exists a shortcut, though.
If you have a simple props made up only of types that implement `FromStr`, then you can derive the `FromCaptures`.


### Example 
```rust
pub struct MyComponent;

#[derive(FromCaptures, Properties)]
pub struct MyComponentProps;

impl Component for MyComponent {
   type Properties = MyComponentProps;
   // ...
}

// ...

html! {
    <Router>
        <Route matcher=route!("/a/{capture}") render=component::<MyComponent>() />
    </Router>
}
```

### Note
The derive functionality of `FromCaptures` is relatively basic.
It cannot handle `Option`s that you might want to populate based on optional matching sections (`()`).
It is recommended that you implement `FromCaptures` yourself for `Properties` structs that contain types that aren't automatically convertible from Strings.


