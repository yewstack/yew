#![no_implicit_prelude]
#![recursion_limit = "128"]

mod t1 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<T: ::std::clone::Clone + ::std::default::Default + ::std::cmp::PartialEq> {
        #[prop_or_default]
        value: T,
    }

    fn optional_prop_generics_should_work() {
        use ::yew::Properties;

        Props::<bool>::builder().build();
        Props::<bool>::builder().value(true).build();
    }
}

mod t2 {
    #[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
    struct Value;
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<T: ::std::clone::Clone + ::std::cmp::PartialEq> {
        value: T,
    }

    fn required_prop_generics_should_work() {
        use ::yew::Properties;

        Props::<Value>::builder().value(Value).build();
    }
}

mod t3 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props {
        b: i32,
        #[prop_or_default]
        a: i32,
    }

    fn order_is_alphabetized() {
        use ::yew::Properties;

        Props::builder().b(1).build();
        Props::builder().a(1).b(2).build();
    }
}

mod t4 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<T>
    where
        T: ::std::clone::Clone + ::std::default::Default + ::std::cmp::PartialEq,
    {
        #[prop_or_default]
        value: T,
    }

    fn optional_prop_generics_should_work() {
        use ::yew::Properties;

        Props::<bool>::builder().build();
        Props::<bool>::builder().value(true).build();
    }
}

mod t5 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<
        'a,
        T: ::std::clone::Clone + ::std::default::Default + ::std::cmp::PartialEq + 'a,
    > {
        #[prop_or_default]
        static_value: &'static str,
        value: &'a T,
    }

    fn optional_prop_generics_with_lifetime_should_work() {
        use ::std::{convert::From, string::String};
        use ::yew::Properties;

        Props::<String>::builder().value(&String::from("")).build();
        Props::<String>::builder()
            .static_value("")
            .value(&String::from(""))
            .build();
    }
}

mod t6 {
    #[derive(::yew::Properties, ::std::clone::Clone, ::std::cmp::PartialEq)]
    pub struct Props<T: ::std::str::FromStr + ::std::clone::Clone + ::std::cmp::PartialEq>
    where
        <T as ::std::str::FromStr>::Err: ::std::clone::Clone + ::std::cmp::PartialEq,
    {
        value: ::std::result::Result<T, <T as ::std::str::FromStr>::Err>,
    }

    fn required_prop_generics_with_where_clause_should_work() {
        use ::std::{convert::From, result::Result::Ok, string::String};
        use ::yew::Properties;

        Props::<String>::builder()
            .value(Ok(String::from("")))
            .build();
    }
}

mod t7 {
    #[derive(::std::clone::Clone, Debug, Eq, ::std::cmp::PartialEq)]
    pub enum Foo {
        One,
        Two,
    }

    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props {
        #[prop_or(Foo::One)]
        value: Foo,
    }

    fn prop_or_value_should_work() {
        use ::std::assert_eq;
        use ::yew::Properties;

        let props = Props::builder().build();
        assert_eq!(props.value, Foo::One);
        Props::builder().value(Foo::Two).build();
    }
}

mod t8 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props {
        #[prop_or_else(|| 123)]
        value: i32,
    }

    fn prop_or_else_closure_should_work() {
        use ::std::assert_eq;
        use ::yew::Properties;

        let props = Props::builder().build();
        assert_eq!(props.value, 123);
        Props::builder().value(123).build();
    }
}

mod t9 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<T: ::std::str::FromStr + ::std::clone::Clone + ::std::cmp::PartialEq>
    where
        <T as ::std::str::FromStr>::Err: ::std::clone::Clone + ::std::cmp::PartialEq,
    {
        #[prop_or_else(default_value)]
        value: ::std::result::Result<T, <T as ::std::str::FromStr>::Err>,
    }

    fn default_value<T: ::std::str::FromStr + ::std::clone::Clone>(
    ) -> ::std::result::Result<T, <T as ::std::str::FromStr>::Err>
    where
        <T as ::std::str::FromStr>::Err: ::std::clone::Clone,
    {
        "123".parse()
    }

    fn prop_or_else_function_with_generics_should_work() {
        use ::std::{assert_eq, result::Result::Ok};
        use ::yew::Properties;

        let props = Props::<i32>::builder().build();
        assert_eq!(props.value, Ok(123));
        Props::<i32>::builder().value(Ok(456)).build();
    }
}

mod t10 {
    // this test makes sure that Yew handles generic params with default values properly.

    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Foo<S, M = S>
    where
        S: ::std::clone::Clone + ::std::cmp::PartialEq,
        M: ::std::clone::Clone + ::std::cmp::PartialEq,
    {
        bar: S,
        baz: M,
    }
}

mod t11 {
    // this test makes sure that Yew handles generic params with const generics properly.

    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Foo<T, const N: usize>
    where
        T: ::std::clone::Clone + ::std::cmp::PartialEq,
    {
        bar: [T; N],
    }
}

mod t12 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<T: ::std::clone::Clone + ::std::cmp::PartialEq> {
        value: ::std::option::Option<T>,
    }

    fn optional_prop_generics_should_work() {
        use ::yew::Properties;

        Props::<bool>::builder().build();
        Props::<bool>::builder().value(true).build();
    }
}

fn main() {}
