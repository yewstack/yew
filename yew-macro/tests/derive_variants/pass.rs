#![recursion_limit = "128"]

use yew::prelude::*;

use yew::html::ChildrenRenderer;
use yew::virtual_dom::{VChild, VComp};

mod t1 {
    use super::*;

    #[derive(Clone, Properties)]
    pub struct MyFirstComponentProps {
        #[prop_or_default]
        pub foo: String,
    }

    pub struct MyFirstComponent;

    impl Component for MyFirstComponent {
        type Properties = MyFirstComponentProps;
        type Message = ();
        fn create(_: MyFirstComponentProps, _: ComponentLink<Self>) -> Self { unimplemented!() }
        fn change(&mut self, _: MyFirstComponentProps) -> ShouldRender { unimplemented!() }
        fn update(&mut self, _: ()) -> ShouldRender { unimplemented!() }
        fn view(&self) -> Html { unimplemented!() }
    }

    #[derive(Clone, Properties)]
    pub struct MySecondComponentProps {
        #[prop_or_default]
        pub bar: String,
    }

    pub struct MySecondComponent;

    impl Component for MySecondComponent {
        type Properties = MySecondComponentProps;
        type Message = ();
        fn create(_: MySecondComponentProps, _: ComponentLink<Self>) -> Self { unimplemented!() }
        fn change(&mut self, _: MySecondComponentProps) -> ShouldRender { unimplemented!() }
        fn update(&mut self, _: ()) -> ShouldRender { unimplemented!() }
        fn view(&self) -> Html { unimplemented!() }
    }

    #[derive(Clone, Variants)]
    pub enum MyVariants {
        MyFirstComponent(MyFirstComponentProps),
        MySecondComponent(MySecondComponentProps),
    }

    #[derive(Clone)]
    pub struct MyItem {
        props: MyVariants,
    }

    impl<CHILD> From<VChild<CHILD>> for MyItem
    where
        CHILD: Component,
        CHILD::Properties: Into<MyVariants>
    {
        fn from(vchild: VChild<CHILD>) -> Self {
            Self {
                props: vchild.props.into(),
            }
        }
    }

    impl Into<Html> for MyItem {
        fn into(self) -> Html {
            match self.props {
                MyVariants::MyFirstComponent(props) => VComp::new::<MyFirstComponent>(props, NodeRef::default(), None).into(),
                MyVariants::MySecondComponent(props) => VComp::new::<MySecondComponent>(props, NodeRef::default(), None).into(),
            }
        }
    }

    #[derive(Clone, Properties)]
    pub struct MyListProps {
        #[prop_or_default]
        children: ChildrenRenderer<MyItem>,
    }

    pub struct MyList {
        props: MyListProps,
    }

    impl Component for MyList {
        type Properties = MyListProps;
        type Message = ();
        fn create(_: MyListProps, _: ComponentLink<Self>) -> Self { unimplemented!() }
        fn change(&mut self, _: MyListProps) -> ShouldRender { unimplemented!() }
        fn update(&mut self, _: ()) -> ShouldRender { unimplemented!() }
        fn view(&self) -> Html {
            html! {
                <div class = "my-list">
                    { self.props.children.clone() }
                </div>
            }
        }
    }
}

mod t2 {
    use super::*;

    #[derive(Clone, Properties)]
    pub struct MyFirstComponentProps<T: Clone + Default> {
        #[prop_or_default]
        pub foo: T,
    }

    pub struct MyFirstComponent<T: 'static + Clone + Default> { props: MyFirstComponentProps<T>, }

    impl<T: 'static + Clone + Default> Component for MyFirstComponent<T> {
        type Properties = MyFirstComponentProps<T>;
        type Message = ();
        fn create(_: MyFirstComponentProps<T>, _: ComponentLink<Self>) -> Self { unimplemented!() }
        fn change(&mut self, _: MyFirstComponentProps<T>) -> ShouldRender { unimplemented!() }
        fn update(&mut self, _: ()) -> ShouldRender { unimplemented!() }
        fn view(&self) -> Html { unimplemented!() }
    }

    #[derive(Clone, Properties)]
    pub struct MySecondComponentProps<T: Clone + Default> {
        #[prop_or_default]
        pub bar: T,
    }

    pub struct MySecondComponent<T: 'static + Clone + Default> { props: MySecondComponentProps<T>, }

    impl<T: 'static + Clone + Default> Component for MySecondComponent<T> {
        type Properties = MySecondComponentProps<T>;
        type Message = ();
        fn create(_: MySecondComponentProps<T>, _: ComponentLink<Self>) -> Self { unimplemented!() }
        fn change(&mut self, _: MySecondComponentProps<T>) -> ShouldRender { unimplemented!() }
        fn update(&mut self, _: ()) -> ShouldRender { unimplemented!() }
        fn view(&self) -> Html { unimplemented!() }
    }

    #[derive(Clone, Variants)]
    pub enum MyVariants<T: 'static + Clone + Default> {
        MyFirstComponent(MyFirstComponentProps<T>),
        MySecondComponent(MySecondComponentProps<T>),
    }

    #[derive(Clone)]
    pub struct MyItem<T: 'static + Clone + Default> {
        props: MyVariants<T>,
    }

    impl<CHILD, T: 'static + Clone + Default> From<VChild<CHILD>> for MyItem<T>
    where
        CHILD: Component,
        CHILD::Properties: Into<MyVariants<T>>,
    {
        fn from(vchild: VChild<CHILD>) -> Self {
            Self {
                props: vchild.props.into(),
            }
        }
    }

    impl<T: 'static + Clone + Default> Into<Html> for MyItem<T> {
        fn into(self) -> Html {
            match self.props {
                MyVariants::MyFirstComponent(props) => VComp::new::<MyFirstComponent<T>>(props, NodeRef::default(), None).into(),
                MyVariants::MySecondComponent(props) => VComp::new::<MySecondComponent<T>>(props, NodeRef::default(), None).into(),
            }
        }
    }

    #[derive(Clone, Properties)]
    pub struct MyListProps {
        #[prop_or_default]
        children: ChildrenRenderer<MyItem<String>>,
    }

    pub struct MyList {
        props: MyListProps,
    }

    impl Component for MyList {
        type Properties = MyListProps;
        type Message = ();
        fn create(_: MyListProps, _: ComponentLink<Self>) -> Self { unimplemented!() }
        fn change(&mut self, _: MyListProps) -> ShouldRender { unimplemented!() }
        fn update(&mut self, _: ()) -> ShouldRender { unimplemented!() }
        fn view(&self) -> Html {
            html! {
                <div class = "my-list">
                    { self.props.children.clone() }
                </div>
            }
        }
    }
}

fn main() {}

