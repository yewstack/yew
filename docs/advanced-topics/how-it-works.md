---
description: Low level details about the framework
---

# Low-level library internals

Component-lifecycle state machine, vdom diff algorithm.

## Under the hood of the `html!` macro

**A quick refresher on macros (feel free to skip this if you're confident with Rust's macros)**
Rust's macros are an example of "metaprogramming" - the term is the Greek "meta" combined with 
"programming", although the meaning of "meta" in this context is not the Greek one but rather its
meaning in epistemology (the study of knowledge) which means about. Metaprogramming is "programming"
about programming. In this context this is about computer programs which write other computer
programs though metaprogramming doesn't have to explicitly be about writing programs.

The `html!` macro turns code written in a custom HTML-like syntax into valid Rust code. We'll use
the following code (taken from the `dashboard` example) as a demo of what the `html!` macro does.
Note that to make this code more parsimonious this is an abridged version of the code in that
example.

```rust
html! {
    <div>
        <nav class="menu">
            <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, false))>
                { "Fetch Data" }
            </button>
            <button onclick=self.link.callback(|_| Msg::FetchData(Format::Json, true))>
                { "Fetch Data [binary]" }
            </button>
        </nav>
    </div>
}
```

There's a useful tool called `cargo expand` which allows you to see the expansion of Rust macros.
If you're confused as to why Yew is doing something, expanding the `html!` macro calls in your
program can give you an insight into why something is happening. `cargo expand` isn't shipped with
`cargo` by default so you'll need to install it with `cargo install cargo-expand` if you haven't
already. We're going to give an example of what the output code might look like and how to navigate
it.

After expanding this macro we get an output which looks like this. Note that some additional code
which the macro also outputs has been stripped out; that code is necessary because Rust only allows
macros to be used in an item position but we're trying to output expressions – the code not shown 
here is a hack to get around that. Also note that this code has been tidied up using `rustfmt` to
make it display in a more readable format.

Some things that might appear strange in this code are:
1. Instead of using `yew::<whatever>` it uses `::yew::<whatever>` - this is to make sure that the
Yew package is referenced correctly. This is also why `::alloc::vec::Vec::new()` is called instead
of just `Vec::new()`.
2. All the variable names are prefixed with `__` – this is to ensure that they don't conflict with
other names you might have defined.
3. `<identifier as TraitName>` – this is just a way to make sure that we're using items from the
correct trait (and not a different trait, if a different trait with the same item has been defined).

The explanation for what's going on is interwoven into the code as comments.

```rust
::yew::virtual_dom::VNode::from({
    // this creates the root tag (in this case a "div")
    let mut __yew_vtag = ::yew::virtual_dom::VTag::new("div");
    // here we add any attributes needed to the tag – in this case there aren't any so we just use
    // an empty `Vec`
    __yew_vtag.add_attributes(::alloc::vec::Vec::new());
    // here we add any event listeners needed to the tag – in this case there aren't any so we just use
    // an empty `Vec`
    __yew_vtag.add_listeners(::alloc::vec::Vec::new());
    // here we add all the children to the parent tag
    __yew_vtag.add_children(<[_]>::into_vec(box [({
        // we create a new tag here (the "nav" tag)
        let mut __yew_vtag = ::yew::virtual_dom::VTag::new("nav");
        /* 
        Add CSS classes for the tag. This code is a little bit confusing at first, but it's not
        too bad. First note that this is the same as
        ```
        yew::virtual_dom::Classes::from("menu");
        ```
        `Classes` is a thin wrapper around a `HashSet` and handles manipulation of CSS classes.
        */
        let __yew_classes = ::std::convert::Into::<::yew::virtual_dom::Classes>::into("menu");
        if !__yew_classes.is_empty() {
            // if there are any CSS classes then they should be applied to the DOM
            // if there aren't, then they aren't applied (in the interest of efficiency)
            __yew_vtag.add_attribute("class", &__yew_classes);
        }
        // add attributes and listeners
        __yew_vtag.add_attributes(::alloc::vec::Vec::new());
        __yew_vtag.add_listeners(::alloc::vec::Vec::new());
        // add the children (a pattern seems to be emerging here)
        __yew_vtag.add_children({
            // create a new `Vec` which will store all the child nodes
            let mut __yew_v = ::std::vec::Vec::new();
            // `push` each child onto the `Vec`
            __yew_v.push(
                ({
                    let mut __yew_vtag = ::yew::virtual_dom::VTag::new("button");
                    // you know the drill by now – add attributes and listeners...
                    __yew_vtag.add_attributes(::alloc::vec::Vec::new());
                    __yew_vtag.add_listeners(<[_]>::into_vec(box [::std::rc::Rc::new({
                        ::yew::html::onclick::Wrapper::new(
                            <::yew::virtual_dom::VTag as ::yew::virtual_dom::Transformer<
                                _,
                                _,
                            >>::transform(
                                self.link.callback(|_| Msg::FetchData(Format::Json, false)),
                            ),
                        )
                    })]));
                    // ... and the child nodes
                    __yew_vtag.add_children(<[_]>::into_vec(box [("Fetch Data").into()]));
                    // return this is a `VNode` using the `From` trait
                    ::yew::virtual_dom::VNode::from(__yew_vtag)
                })
                .into(),
            );
            __yew_v.push(
                ({
                    let mut __yew_vtag = ::yew::virtual_dom::VTag::new("button");
                    __yew_vtag.add_attributes(::alloc::vec::Vec::new());
                    // here some event listeners are added
                    __yew_vtag.add_listeners(<[_]>::into_vec(box [::std::rc::Rc::new({
                        ::yew::html::onclick::Wrapper::new(
                            // `Transformer` is a trait which turns an input into an output
                            <::yew::virtual_dom::VTag as ::yew::virtual_dom::Transformer<
                                // here we elide the type parameter (they are inferred later)
                                _,
                                _,
                            >>::transform(
                                // register a callback
                                self.link.callback(|_| Msg::FetchData(Format::Json, true)),
                            ),
                        )
                    })]));
                    __yew_vtag
                        .add_children(<[_]>::into_vec(box [("Fetch Data [binary]").into()]));
                    ::yew::virtual_dom::VNode::from(__yew_vtag)
                })
                .into(),
            );
            __yew_v
        });
        ::yew::virtual_dom::VNode::from(__yew_vtag)
    })
    .into()]));
    ::yew::virtual_dom::VNode::from(__yew_vtag)
})
```

The code the macro outputs is, in the end, computer generated code and isn't ever going to be
particularly beautiful or elegant. One thing of note, however is how much easier it is to write
code using the `html!` macro rather than doing all that repetitive work yourself. Part of this is
because the `html!` macro allows you to write code in a declarative style (i.e. you specify what
the end result should be) rather than writing code in an imperative style where you'd write code
which specifies all the steps to get to the end result. Having tools like the `html!` macro becomes
increasingly useful as your codebase gets larger and more people work on it.

When looking through code output by the macro the [API documentation](https://docs.rs/yew) is a
useful tool for working out what different items of code are doing. Protip: the documentation is
searchable!

## What is a virtual DOM?

The DOM ("document object model") provides a representation of the HTML content on a web page in a
way that programs can understand. "Object" is about representing the DOM as "objects" so that a web
page can be modified by a programming language.

A while ago, a development team at Facebook encountered a problem. Before frameworks like React
existed, everyone built web applications by directly manipulating the DOM. This was error prone and
lead to hard to maintain code. For example, consider the following Javascript code, which loads
some blog posts from a fictional API.

```javascript
function reloadBlogPosts() {
    fetch("https://api.example.com/blog").then(request => {
        if (request.ok) {
            // let's assume that there's already an element '#blogposts' in the DOM
            let blogPostNode = document.getElementById("#blogposts");
            // clear the existing contents of the node
            blogPostNode.innerHTML = "";
            // add the new data into the DOM
            request.json().then(data => {
                data.map(blogPost => {
                       blogPostNode.appendChild(
                         document.createElement("h1")
                                 .appendChild(document.createTextNode(blogPost.title))
                       );
                })
            })
        }
    })
}
```

This isn't nice code, and it certainly wouldn't make you popular with your users (because of the 
poor performance and spinning fans it would cause). Every time you want to reload the blog posts it 
has to redraw every single blog post. This is slow; we have to do a lot of updates to content in the 
DOM (which is slow to do because the browser has to do a lot of work to update DOM nodes). The team 
at Facebook found that it was significantly faster to compare the contents of the nodes they wanted 
to create with the existing nodes in the DOM before only updating the contents of the nodes which 
needed to be updated rather than rewriting everything.

This is called a "virtual DOM" because there's two DOM trees – one which is updated by the
application and the real DOM, which a bit of code will compare to the virtual DOM and update only
the items which differ between the real and virtual DOMs. This gave birth to the now ubiquitous 
"React" web framework.

## Yew's virtual DOM demystified

*Contribute to the docs – explain how `yew::virtual_dom` works in depth*

## Further reading
* [More information about macros from the Rust Book](https://doc.rust-lang.org/stable/book/ch19-06-macros.html)
* [More information about `cargo-expand`](https://github.com/dtolnay/cargo-expand)
* [The API documentation for `yew::virtual_dom`](https://docs.rs/yew/*/yew/virtual_dom/index.html)