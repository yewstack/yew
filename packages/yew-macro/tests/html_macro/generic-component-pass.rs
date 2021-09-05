#![no_implicit_prelude]

pub struct Generic<T> {
    marker: ::std::marker::PhantomData<T>,
}

impl<T> ::yew::Component for Generic<T>
where
    T: 'static,
{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

pub struct Generic2<T1, T2> {
    marker: ::std::marker::PhantomData<(T1, T2)>,
}

impl<T1, T2> ::yew::Component for Generic2<T1, T2>
where
    T1: 'static,
    T2: 'static,
{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &::yew::Context<Self>) -> Self {
        ::std::unimplemented!()
    }
    fn view(&self, _ctx: &::yew::Context<Self>) -> ::yew::Html {
        ::std::unimplemented!()
    }
}

fn compile_pass() {
    ::yew::html! { <Generic<::std::string::String> /> };
    ::yew::html! { <Generic<::std::string::String> ></Generic<::std::string::String>> };

    ::yew::html! { <Generic<::std::vec::Vec<::std::string::String>> /> };
    ::yew::html! { <Generic<::std::vec::Vec<::std::string::String>>></ Generic<::std::vec::Vec<::std::string::String>>> };

    ::yew::html! { <Generic<usize> /> };
    ::yew::html! { <Generic<usize>></Generic<usize>> };
    ::yew::html! { <Generic<::std::string::String, > /> };
    ::yew::html! { <Generic<::std::string::String, >></Generic<::std::string::String,>> };

    ::yew::html! { <Generic2<::std::string::String, ::std::string::String> /> };
    ::yew::html! { <Generic2<::std::string::String, ::std::string::String>></Generic2<::std::string::String, ::std::string::String>> };

    ::yew::html! { <Generic2<::std::string::String, ::std::string::String, > /> };
    ::yew::html! { <Generic2<::std::string::String, ::std::string::String, >></Generic2<::std::string::String, ::std::string::String, >> };
}

fn main() {}
