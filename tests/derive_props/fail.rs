#![recursion_limit = "128"]

use yew::prelude::*;

mod t1 {
    use super::*;
    #[derive(Clone)]
    struct Value;
    #[derive(Clone, Properties)]
    pub struct Props {
        // ERROR: optional params must implement default
        value: Value,
    }
}

mod t2 {
    use super::*;
    #[derive(Clone, Properties)]
    pub struct Props {
        // ERROR: optional is not a tag
        #[props(optional)]
        value: String,
    }
}

mod t3 {
    use super::*;
    #[derive(Clone, Properties)]
    pub struct Props {
        #[props(required)]
        value: String,
    }

    fn required_props_should_be_set() {
        Props::builder().build();
    }
}

mod t4 {
    use super::*;
    #[derive(Clone, Properties)]
    pub struct Props {
        b: i32,
        #[props(required)]
        a: i32,
    }

    fn enforce_ordering() {
        Props::builder().b(1).a(2).build();
    }
}

mod t5 {
    use super::*;
    #[derive(Clone, Properties)]
    pub struct Props {
        // ERROR: default must be given a value
        #[props(default)]
        value: String,
    }
}

mod t6 {
    use super::*;
    #[derive(Clone, Properties)]
    pub struct Props {
        // ERROR: the value must be a string
        #[props(default = 123)]
        value: i32,
    }
}

mod t7 {
    use super::*;
    #[derive(Clone, Properties)]
    pub struct Props {
        // ERROR: the value must be parsed as a String
        #[props(default = "123")]
        value: String,
    }
}

fn main() {}
