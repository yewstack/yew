use crate::random;
use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;

use fake::faker::address::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;

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
            let no = random::range_exclusive(1, 300);
            let state = StateAbbr(EN).fake::<String>();
            let city = CityName(EN).fake::<String>();
            let street = StreetName(EN).fake::<String>();

            Rc::from(format!("{} {} St., {}, {}", no, street, city, state).as_str())
        };

        Self {
            id,
            name: Rc::from(Name(EN).fake::<String>().as_str()),
            age: random::range_exclusive(7, 77),
            address,
        }
    }

    fn render(&self) -> Html {
        html! {
            <div class="card w-50 card_style">
                <div class="card-body">
                    <h5 class="card-title">{ format!("{} - {}", &self.id, &self.name) }</h5>
                    <p class="card-text">{ format!("Age: {}", &self.age) }</p>
                    <p class="card-text">{ format!("Address: {}", &self.address) }</p>
                </div>
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
            <div class="text-info" id=self.info.id.to_string()>
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
                        <div key=info.id.to_string() class="text-danger" id=info.id.to_string()>
                            { info.render() }
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-danger" id=info.id.to_string()>
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
