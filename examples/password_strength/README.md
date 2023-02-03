# Password Strength Estimator Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fpassword_strength)](https://examples.yew.rs/password_strength)

A password strength estimator implemented in Yew.

## Running

Run a debug version of this application:

```bash
trunk serve --open
```

### Notes

If this example is a bit slow, you sould try running it with the `release` profile.

## Concepts

This example

- makes use of controlled components.
- extracts new value from `InputEvent`
- calls out to `js_sys` to invoke a foreign function, `Math.random()`
