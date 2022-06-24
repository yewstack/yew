#![recursion_limit = "128"]

use yew::prelude::*;

mod t1 {
    use super::*;
    #[derive(Clone)]
    struct Value;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: optional params must implement default
        #[prop_or_default]
        value: Value,
    }
}

mod t2 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: old syntax no longer supported
        #[props(default)]
        value: String,
    }
}

mod t3 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        value: String,
    }

    fn required_props_should_be_set() {
        ::yew::props!{ Props { } };
    }
}

mod t5 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: prop_or must be given a value
        #[prop_or()]
        value: String,
    }
}

mod t6 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: 123 is not a String
        #[prop_or(123)]
        value: String,
    }
}

mod t7 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: 123 is not a function
        #[prop_or_else(123)]
        value: i32,
    }
}

mod t8 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: cannot find function foo in this scope
        #[prop_or_else(foo)]
        value: String,
    }
}

mod t9 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: the function must take no arguments
        #[prop_or_else(foo)]
        value: String,
    }

    fn foo(bar: i32) -> String {
        unimplemented!()
    }
}

mod t10 {
    use super::*;
    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        // ERROR: the function returns incompatible types
        #[prop_or_else(foo)]
        value: String,
    }

    fn foo() -> i32 {
        unimplemented!()
    }
}

fn main() {}
