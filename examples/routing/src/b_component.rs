
use router;
use router::Route;
use yew::prelude::*;
use std::usize;


pub struct BModel {
    number: Option<usize>,
    sub_path: Option<String>,
    router: Box<Bridge<router::Router<()>>>
}

pub enum Msg {
    Navigate(Vec<Msg>), // Navigate after performing other actions
    Increment,
    Decrement,
    UpdateSubpath(String),
    HandleRoute(Route<()>)
}


impl Component for BModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {

        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);

        router.send(router::Request::GetCurrentRoute);

        BModel {
            number: None,
            sub_path: None,
            router
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Navigate(msgs) => {
                // Perform the wrapped action first
                for msg in msgs{
                    self.update(msg);
                }

                // The path dictating that this component be instantiated must be provided
                let mut path_segments = vec!["b".into()];
                if let Some(ref sub_path) = self.sub_path {
                    path_segments.push(sub_path.clone())
                }

                let fragment: Option<String> = self.number.map(|x: usize | x.to_string());

                let route = router::Route {
                    path_segments,
                    query: None,
                    fragment,
                    state: (),
                };

                self.router.send(router::Request::ChangeRoute(route));
                false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                // Instead of each component selecting which parts of the path are important to it,
                // it is also possible to match on the `route.to_route_string().as_str()` once
                // and create enum variants representing the different children and pass them as props.
                self.sub_path = route.path_segments.get(1).map(String::clone);
                self.number = route.fragment.and_then(|x| usize::from_str_radix(&x, 10).ok());

                true
            }
            Msg::Increment => {
                let n = if let Some(number) = self.number{
                    number + 1
                } else {
                    1
                };
                self.number = Some(n);
                true
            }
            Msg::Decrement => {
                let n: usize = if let Some(number) = self.number{
                    if number > 0 {
                        number - 1
                    } else {
                        number
                    }
                } else {
                    0
                };
                self.number = Some(n);
                true
            }
            Msg::UpdateSubpath(path) => {
                self.sub_path = Some(path);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Apparently change MUST be implemented in this case, even though no props were changed
        true
    }
}
impl Renderable<BModel> for BModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    { self.display_number() }
                    <button onclick=|_| Msg::Navigate(vec![Msg::Increment]),>{ "Increment" }</button>
                    <button onclick=|_| Msg::Navigate(vec![Msg::Decrement]),>{ "Decrement" }</button>
                </div>

                { self.display_subpath_input() }

            </div>
        }
    }
}

impl BModel {
    fn display_number(&self) -> String {
        if let Some(number) = self.number {
            format!("Number: {}", number)
        } else {
            format!("Number: None")
        }
    }
    fn display_subpath_input(&self) -> Html<BModel> {
        let sub_path = self.sub_path.clone();
        html! {
            <input
                placeholder="subpath",
                value=sub_path.unwrap_or("".into()),
                oninput=|e| Msg::Navigate(vec![Msg::UpdateSubpath(e.value)]),
                />
        }
    }
}