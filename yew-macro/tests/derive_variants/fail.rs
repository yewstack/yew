#![recursion_limit = "128"]

use yew::prelude::*;

mod t1 {
    use super::*;

    // ERROR: only enums are supported.
    #[derive(Clone, Variants)]
    pub struct MyVariants;
}

mod t2 {
    use super::*;

    #[derive(Clone, Properties)]
    pub struct MyFirstComponentProps {
        #[prop_or_default]
        pub foo: String,
    }

    #[derive(Clone, Variants)]
    pub enum MyVariants {
        // ERROR: only unnamed fields are supported
        MyFirstComponent { x: MyFirstComponentProps, }
    }
}

mod t3 {
    use super::*;

    #[derive(Clone, Properties)]
    pub struct MyFirstComponentProps {
        #[prop_or_default]
        pub foo: String,
    }

    #[derive(Clone, Variants)]
    pub enum MyVariants {
        // ERROR: only fields with a single field are supported
        MyFirstComponent(MyFirstComponentProps, MyFirstComponentProps),
    }
}

fn main() {}

