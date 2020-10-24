use crate::random;
use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersonInfo {
    pub id: usize,
    pub name: Rc<str>,
    pub address: Rc<str>,
    pub age: usize,
}
impl PersonInfo {
    pub fn new_random(id: usize) -> Self {
        let address = {
            let count = random::range_exclusive(3, 6);
            Rc::from(random::words(count, 5, 12).join(" ").as_str())
        };

        Self {
            id,
            name: Rc::from(random::words(2, 4, 15).join(" ").as_str()),
            age: random::range_exclusive(7, 77),
            address,
        }
    }

    fn render(&self) -> Html {
        html! {
            <div class="person">
                <h1>{ &self.id }{ " - " }{ &self.name }</h1>
                <p>{ "Age: " }{ &self.age }</p>
                <p>{ "Address: " }{ &self.address }</p>
            </div>
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PersonComponent {
    info: PersonInfo,
}
impl Component for PersonComponent {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="component-person" id=self.info.id.to_string()>
                { self.info.render() }
            </div>
        }
    }
}

pub enum PersonType {
    Inline(PersonInfo),
    Component(PersonInfo),
}
impl PersonType {
    pub fn info(&self) -> &PersonInfo {
        match self {
            Self::Inline(info) => info,
            Self::Component(info) => info,
        }
    }

    pub fn new_random(id: usize, ratio: f64) -> Self {
        let info = PersonInfo::new_random(id);
        if random::chance(ratio) {
            Self::Inline(info)
        } else {
            Self::Component(info)
        }
    }

    pub fn render(&self, keyed: bool) -> Html {
        match self {
            Self::Inline(info) => {
                if keyed {
                    html! {
                        <div key=info.id.to_string() class="basic-person" id=info.id.to_string()>
                            { info.render() }
                        </div>
                    }
                } else {
                    html! {
                        <div class="basic-person" id=info.id.to_string()>
                            { info.render() }
                        </div>
                    }
                }
            }
            Self::Component(info) => {
                if keyed {
                    html! { <PersonComponent key=info.id.to_string() info=info /> }
                } else {
                    html! { <PersonComponent info=info /> }
                }
            }
        }
    }
}
