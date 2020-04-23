# Router Component

The `Router` component is used to contain `Route` components.
The `Route` components allow you to specify which routes to match, and what to render when they do.


## Logic
The `Router`, when routing, wants to find a valid target.
To do this, it will look at each of its child `Route` components.
For each `Route` component, the `Router` will attempt to match its route string against the `Route`'s matcher.
If the matcher succeeds, then a `Matches` (alias to `HashMap<&str, String>`) is produced and fed to its render function (if one is provided).
If the render function returns None, then the `Router` will continue to look an the next `Route`, but if `Some` is returned, it has completed its task and will cease looking for targets.

#### Render
If the `render` property of the `Route` component is specified, it call that function to get content to display.
The signature of this function is `fn(matches: &Matches) -> Option<Html<Router>>`. 
The `Router` will only cease its search for a target if this function returns `Some`, otherwise it will continue to try other `Route`s.

The `component()` function allows you to specify a component to attempt render.
You can only call this with a type parameter of a component whose `Properties` have implemented `FromCaptures`.

Alternatively, `render()` can be called instead, which takes a closure that returns an `Option<Html<_>>`.

#### Children
If the match succeeds and the `Route` specified `children` instead of a `render` prop, the children will always be displayed.
Rendering children may be more ergonomic, but you loose access to the `&Matches` produced by the `Route`'s matcher, and as consequence you lose the ability to conditionally render

#### Both
If both a render prop and children are provided, they will both render, as long as the render function returns `Some`. 
If it returns `None`, then neither will be displayed and the `Router` will continue to search for a target.

#### Neither
If neither are provided, obviously nothing will be rendered, and the search for a target will continue.

### Example
```rust
html! {
    <Router>
        <Route matcher=route!("/a") render=component::<AModel>() />
        <Route matcher=route!("/b")> 
            <BModel/>
        </Route>
        <Route matcher=route!("/c") /> // Will never render. 
        <Route matcher=route!("/d") render=component::<DModel>() > // DModel will render above the EModel component.
            <EModel />
        </Route> 
    </Router>
}
```


## Ordering
Since you can create `Route`s that have matchers that can both match a given route string, you should put the more specific one above the more general one.
This ensures that the specific case has a chance to match first.

Additionally, using `{*}` or `{*:name}` in the last `Route` is a good way to provide a default case.

### Example
```rust
html! {
    <Router>
        <Route matcher=route!("/a/specific/path") render=component::<AModel>() />
        <Route matcher=route!("/a/{}/{}") render=component::<BModel>() /> // will match any valid url that has 3 sections, and starts with `/a/` and is not `/a/specific/path`
        <Route matcher=route!("/a/path/{}") render=component::<CModel>() /> // Will never match
        <Route matcher=route!("{*}") render=component::<DModel>() /> // Will match anything that doesn't match above.
    </Router>
}
```