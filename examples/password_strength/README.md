# Password Strength Estimator Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fpassword_strength)](https://examples.yew.rs/password_strength)

A password strength estimator implemented in Yew.

## Running

You should run this example with the `--release` flag:

```bash
trunk serve --release
```

## Concepts

This example

- makes use of controlled components.
- extracts new value from `InputEvent`
- calls out to `js_sys` to invoke a foreign function, `Math.random()`
