#![allow(clippy::needless_doctest_main)]
pub mod tutorial;

include!(concat!(env!("OUT_DIR"), "/website_tests.rs"));
