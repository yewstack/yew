use yew::prelude::*;

#[derive(Clone, Properties, Debug, Default, PartialEq)]
pub struct ChildProperties {
    #[prop_or_default]
    pub string: String,
    pub int: i32,
    #[prop_or_default]
    pub opt_str: Option<String>,
    #[prop_or_default]
    pub vec: Vec<i32>,
    #[prop_or_default]
    pub optional_callback: Option<Callback<()>>,
}

fn main() {
    let props = yew::props!(ChildProperties int=5 opt_str="Hello World!");
}
