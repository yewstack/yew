# PR

This is just the file I'm using to keep track of everything. It won't be part of the final PR.

## TODO

- [ ] update all examples
- [ ] update examples for yew-router (fix #1377)
- [ ] update examples for yewtil

## Somewhat related issues

- #1073 (should probably be closed now)
- #1016 (promises some good stylin' for the examples but it's been a while)

## List of examples that I deem satisfactory

- game_of_life
- nested_list
- todomvc

## Examples

### 1. counter

### 2. crm

Modified to make the example more modular.
What used to be a single component handling a lot of message spaghetti is now split into dedicated components.

### 3. custom_components

Should be updated or removed

### 4. dashboard

### 5. file_upload

### 6. fragments

Should be updated or removed

### 7. futures

Renamed from "futures_wp".

### 8. game_of_life

Split code across two files.

### 9. inner_html

### 10. js_callback

Make use of wasm-bindgen's ability to automatically load js files without having to include them in the HTML.
Split the bindings into a separate file for clarity.

Trunk Issue: <https://github.com/thedodd/trunk/issues/40>

### 11. keyed_list

Split into multiple files for clarity.

### 12. large_table

This example was removed because it's quite literally just a large table...
"game_of_life" is a far superior but not-quite-as-big table experience and "keyed_list" takes care of the large part.

### 13. minimal

Removed because this example is very similar to "counter" but it can only count to one.
A "minimal" example should either demonstrate every aspect of Yew's API in a small application or be a template.

### 14. minimal_wp

This example was removed because it's the same as "minimal" but for wasm-pack.
Different approaches to building apps should be explored through templates, not the examples.

### 15. mount_point

### 16. multi_thread

Trunk Issue: <https://github.com/thedodd/trunk/issues/46>.

### 17. nested_list

### 18. node_refs

### 19. npm_and_rest

Renamed to "ccxt_and_gravatar" because it has nothing to do with NPM.
Updated the services to be more ergonomic and added display for the Gravatar profile.

Example should be updated or put out of its misery

### 20. pub_sub

Should be updated

### 21. store

### 22. textarea

Removed because it's literally just a textarea.
Other examples like todomvc use similar concepts for something more useful.

### 23. timer

### 24. todomvc

Split `State` into a separate file to make it more manageable.
Updated TodoMVC CSS to latest version.

### 25. two_apps

### 26. webgl
