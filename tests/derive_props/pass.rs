#![recursion_limit = "128"]

use yew::prelude::*;

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

mod t5 {
    use super::*;

    #[derive(Properties)]
    pub struct Props<'a, T: Default + 'a> {
        static_value: &'static str,
        #[props(required)]
        value: &'a T,
    }

    fn optional_prop_generics_with_lifetime_should_work() {
        Props::<String>::builder().value(&String::from("")).build();
        Props::<String>::builder()
            .static_value("")
            .value(&String::from(""))
            .build();
    }
}

mod t6 {
    use super::*;
    use std::str::FromStr;

    #[derive(Properties, Clone)]
    pub struct Props<T: FromStr + Clone>
    where
    <T as FromStr>::Err: Clone,
    {
        #[props(required)]
        value: Result<T, <T as FromStr>::Err>,
    }

    fn required_prop_generics_with_where_clause_should_work() {
        Props::<String>::builder().value(Ok(String::from(""))).build();
    }
}

fn main() {}
