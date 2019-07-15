use yew::prelude::*;

#[derive(Clone, Default, PartialEq)]
pub struct TestProperties {
    pub string: String,
    pub int: i32,
}

pub struct TestComponent;
impl Component for TestComponent {
    type Message = ();
    type Properties = TestProperties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TestComponent
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }
}
