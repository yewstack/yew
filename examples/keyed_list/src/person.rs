use std::rc::Rc;

use fake::faker::address::raw::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use yew::{html, Component, Context, Html, Properties};

use crate::random;

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

            Rc::from(format!("{no} {street} St., {city}, {state}").as_str())
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

#[derive(Debug, Eq, PartialEq, Properties)]
pub struct PersonProps {
    info: PersonInfo,
}

pub struct PersonComponent;

impl Component for PersonComponent {
    type Message = ();
    type Properties = PersonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="text-info" id={ctx.props().info.id.to_string()}>
                { ctx.props().info.render() }
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
                        <div key={info.id.to_string()} class="text-danger" id={info.id.to_string()}>
                            { info.render() }
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-danger" id={info.id.to_string()}>
                            { info.render() }
                        </div>
                    }
                }
            }
            Self::Component(info) => {
                if keyed {
                    html! { <PersonComponent key={info.id.to_string()} info={info.clone()} /> }
                } else {
                    html! { <PersonComponent info={info.clone()} /> }
                }
            }
        }
    }
}
