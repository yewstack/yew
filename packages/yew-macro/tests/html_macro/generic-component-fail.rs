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

fn compile_fail() {
    html! { <Generic<String>> };
    html! { <Generic<String>></Generic> };
    html! { <Generic<String>></Generic<Vec<String>>> };
}

fn main() {}
