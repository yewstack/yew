# Form /w File Upload Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fform_file)](https://examples.yew.rs/form_file)

This example shows some more comlicated interactions between file uploads, forms, and node_refs.

The file selector change disables the form button untill the file is done being processed, at which point it's stashed in App state untill the form is submitted and it's stored in the FileDetails.

## Concepts

Demonstrates reading from files in Yew with the help of [`gloo::file`](https://docs.rs/gloo-file/latest/gloo_file/).
Check the file_upload example for the simpler case of just uploading a file.

## Todo

 - [] disabled form entirely by checking if there are readers left before allowing submit to proceed

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```