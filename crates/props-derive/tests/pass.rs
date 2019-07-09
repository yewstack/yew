#![recursion_limit = "128"]

use yew::html::Properties;
use yew_props_derive::Properties;

mod t1 {
    use super::*;

    #[derive(Properties)]
    pub struct Props<T: Default> {
        value: T,
    }

    fn optional_prop_generics_should_work() {
        Props::<bool>::builder().build();
        Props::<bool>::builder().value(true).build();
    }
}

mod t2 {
    use super::*;

    struct Value;
    #[derive(Properties)]
    pub struct Props<T> {
        #[props(required)]
        value: T,
    }

    fn required_prop_generics_should_work() {
        Props::<Value>::builder().value(Value).build();
    }
}

mod t3 {
    use super::*;

    #[derive(Properties)]
    pub struct Props {
        #[props(required)]
        b: i32,
        a: i32,
    }

    fn order_is_alphabetized() {
        Props::builder().b(1).build();
        Props::builder().a(1).b(2).build();
    }
}

mod t4 {
    use super::*;

    #[derive(Properties)]
    pub struct Props<T>
    where
        T: Default,
    {
        value: T,
    }

    fn optional_prop_generics_should_work() {
        Props::<bool>::builder().build();
        Props::<bool>::builder().value(true).build();
    }
}

fn main() {}
