use yew::prelude::*;
extern crate alloc;

#[derive(Properties, PartialEq)]
struct Props {
    suboptimal1: String,
    suboptimal2: std::string::String,
    suboptimal3: alloc::string::String
}

fn main() {
    compile_error!("This macro call exists to deliberately fail\
                    the compilation of the test so we can verify output of lints");
}
