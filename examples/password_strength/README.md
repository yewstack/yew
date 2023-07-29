# Password Strength Estimator Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fpassword_strength)](https://examples.yew.rs/password_strength)

A password strength estimator implemented in Yew.

### Notes

If this example is a bit slow, you should try running it with the `release` profile.

## Concepts

This example

- makes use of controlled components.
- extracts new value from `InputEvent`
- calls out to `js_sys` to invoke a foreign function, `Math.random()`

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```