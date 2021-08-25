use std::marker::PhantomData;
use yew::prelude::*;

pub struct Generic<T> {
    marker: PhantomData<T>,
}

impl<T> Component for Generic<T>
where
    T: 'static,
{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        unimplemented!()
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        unimplemented!()
    }
}

pub struct Generic2<T1, T2> {
    marker: PhantomData<(T1, T2)>,
}

impl<T1, T2> Component for Generic2<T1, T2>
where
    T1: 'static,
    T2: 'static,
{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        unimplemented!()
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        unimplemented!()
    }
}

fn compile_pass() {
    html! { <Generic<String> /> };
    html! { <Generic<String> ></Generic<String>> };

    html! { <Generic<Vec<String>> /> };
    html! { <Generic<Vec<String>>></ Generic<Vec<String>>> };

    html! { <Generic<usize> /> };
    html! { <Generic<usize>></Generic<usize>> };
    html! { <Generic<String, > /> };
    html! { <Generic<String, >></Generic<String,>> };

    html! { <Generic2<String, String> /> };
    html! { <Generic2<String, String>></Generic2<String, String>> };

    html! { <Generic2<String, String, > /> };
    html! { <Generic2<String, String, >></Generic2<String, String, >> };
}

fn main() {}
