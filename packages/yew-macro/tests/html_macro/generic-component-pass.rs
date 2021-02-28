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

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        unimplemented!()
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unimplemented!()
    }
    fn view(&self) -> Html {
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

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        unimplemented!()
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        unimplemented!()
    }
    fn view(&self) -> Html {
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
