#![no_implicit_prelude]
#![recursion_limit = "128"]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

mod t1 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<T: ::std::clone::Clone + ::std::default::Default + ::std::cmp::PartialEq> {
        #[prop_or_default]
        value: T,
    }

    fn optional_prop_generics_should_work() {
        ::yew::props! { Props::<::std::primitive::bool> { } };
        ::yew::props! { Props::<::std::primitive::bool> { value: true } };
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
        ::yew::props! { Props::<Value> { value: Value } };
    }
}

mod t3 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props {
        b: ::std::primitive::i32,
        #[prop_or_default]
        a: ::std::primitive::i32,
    }

    fn order_is_alphabetized() {
        ::yew::props! { Props { b: 1 } };
        ::yew::props! { Props { a: 1, b: 2 } };
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
        ::yew::props! { Props::<::std::primitive::bool> { } };
        ::yew::props! { Props::<::std::primitive::bool> { value: true } };
    }
}

mod t5 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props<
        'a,
        T: ::std::clone::Clone + ::std::default::Default + ::std::cmp::PartialEq + 'a,
    > {
        #[prop_or_default]
        static_value: &'static ::std::primitive::str,
        value: &'a T,
    }

    fn optional_prop_generics_with_lifetime_should_work() {
        use ::std::{convert::From, string::String};

        let borrowed_value = &String::from("");
        ::yew::props! { Props::<String> { value: borrowed_value, } };
        ::yew::props! { Props::<String> { static_value: "", value: borrowed_value, } };
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

        ::yew::props! { Props::<String> { value: Ok(String::from("")) } };
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

        let props = ::yew::props! { Props { } };
        assert_eq!(props.value, Foo::One);
        ::yew::props! { Props { value: Foo::Two } };
    }
}

mod t8 {
    #[derive(::std::clone::Clone, ::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props {
        #[prop_or_else(|| 123)]
        value: ::std::primitive::i32,
    }

    fn prop_or_else_closure_should_work() {
        use ::std::assert_eq;

        let props = ::yew::props! { Props { } };
        assert_eq!(props.value, 123);
        ::yew::props! { Props { value: 123 } };
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

        let props = ::yew::props! { Props::<::std::primitive::i32> { } };
        assert_eq!(props.value, Ok(123));
        ::yew::props! { Props::<::std::primitive::i32> { value: Ok(456) } };
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
        ::yew::props! { Props::<::std::primitive::bool> { } };
        ::yew::props! { Props::<::std::primitive::bool> { value: true } };
    }
}

#[deny(non_snake_case, dead_code)]
mod t13 {
    #[derive(::std::cmp::PartialEq, ::yew::Properties)]
    #[allow(non_snake_case)] // putting this on fields directly does not work, even in normal rust
    struct Props {
        #[allow(dead_code)]
        create_message: ::std::option::Option<bool>,
        NonSnakeCase: u32,
    }
}

mod raw_field_names {
    #[derive(::yew::Properties, ::std::cmp::PartialEq)]
    pub struct Props {
        r#true: u32,
        r#pointless_raw_name: u32,
    }

}

fn main() {}
