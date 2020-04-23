use std::usize;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{agent::RouteRequest, prelude::*};

pub struct BModel {
    props: Props,
    router: Box<dyn Bridge<RouteAgent>>,
    increment: Callback<MouseEvent>,
    decrement: Callback<MouseEvent>,
    update_subpath: Callback<InputData>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub number: Option<usize>,
    pub sub_path: Option<String>,
}

#[derive(Debug, Switch, Clone)]
pub enum BRoute {
    #[to = "/{num}?sup_path={sub_path}"]
    Both(usize, String),
    #[to = "/{num}"]
    NumOnly(usize),
    #[to = "?sub_path={sub_path}"]
    SubPathOnly(String),
    #[to = "/"]
    None,
}

impl Into<Props> for BRoute {
    fn into(self) -> Props {
        match self {
            BRoute::None => Props {
                number: None,
                sub_path: None,
            },
            BRoute::NumOnly(number) => Props {
                number: Some(number),
                sub_path: None,
            },
            BRoute::Both(number, sub_path) => Props {
                number: Some(number),
                sub_path: Some(sub_path),
            },
            BRoute::SubPathOnly(sub_path) => Props {
                number: None,
                sub_path: Some(sub_path),
            },
        }
    }
}

pub enum Msg {
    Navigate(Vec<Msg>), // Navigate after performing other actions
    Increment,
    Decrement,
    UpdateSubpath(String),
    NoOp,
}

impl Component for BModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::NoOp); // TODO use a dispatcher instead.
        let router = RouteAgent::bridge(callback);

        BModel {
            props,
            router,
            increment: link.callback(|_| Msg::Navigate(vec![Msg::Increment])),
            decrement: link.callback(|_| Msg::Navigate(vec![Msg::Decrement])),
            update_subpath: link
                .callback(|e: InputData| Msg::Navigate(vec![Msg::UpdateSubpath(e.value)])),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Navigate(msgs) => {
                // Perform the wrapped updates first
                for msg in msgs {
                    self.update(msg);
                }

                // The path dictating that this component be instantiated must be provided
                let route_string = "/b".to_string();
                let route_string = match &self.props.sub_path {
                    Some(sub_path) => route_string + "?sub_path=" + &sub_path,
                    None => route_string,
                };
                let route_string = match &self.props.number.map(|x: usize| x.to_string()) {
                    Some(number) => route_string + "#" + &number,
                    None => route_string,
                };

                let route = Route::from(route_string);

                // Don't tell the router to alert its subscribers,
                // because the changes made here only affect the current component,
                // so mutation might as well be contained to the core component update loop
                // instead of being sent through the router.
                self.router
                    .send(RouteRequest::ChangeRouteNoBroadcast(route));
                true
            }
            Msg::NoOp => false,
            Msg::Increment => {
                let n = if let Some(number) = self.props.number {
                    number + 1
                } else {
                    1
                };
                self.props.number = Some(n);
                true
            }
            Msg::Decrement => {
                let n: usize = if let Some(number) = self.props.number {
                    if number > 0 {
                        number - 1
                    } else {
                        number
                    }
                } else {
                    0
                };
                self.props.number = Some(n);
                true
            }
            Msg::UpdateSubpath(path) => {
                self.props.sub_path = Some(path);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <div>
                    { self.display_number() }
                    <button onclick=&self.increment>{ "Increment" }</button>
                    <button onclick=&self.decrement>{ "Decrement" }</button>
                </div>

                { self.display_subpath_input() }

            </div>
        }
    }
}

impl BModel {
    fn display_number(&self) -> String {
        if let Some(number) = self.props.number {
            format!("Number: {}", number)
        } else {
            "Number: None".to_string()
        }
    }

    fn display_subpath_input(&self) -> Html {
        let sub_path = self.props.sub_path.clone();
        html! {
            <input
                placeholder="subpath",
                value=sub_path.unwrap_or("".into()),
                oninput=&self.update_subpath
                />
        }
    }
}
