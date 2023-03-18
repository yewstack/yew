# File Upload Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Ffile_upload)](https://examples.yew.rs/file_upload)

This example allows the user to select a file from their file system.
The contents of the selected file are then rendered to the page either as a whole or in chunks.

## Concepts

Demonstrates reading from files in Yew with the help of [`gloo::file`](https://docs.rs/gloo-file/latest/gloo_file/).

## Improvements

- Show a progress bar if the file is read in chunks
- Do something interesting with the uploaded file like displaying pictures
- Improve the presentation of the example with CSS.

## Running

Run this application with the trunk development server:

```bash
trunk serve --open
```