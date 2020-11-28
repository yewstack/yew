use instant::Instant;
use person::PersonType;
use yew::prelude::*;
use yew::web_sys::HtmlElement;
use yewtil::NeqAssign;

mod person;
mod random;

pub enum Msg {
    CreatePersons(usize),
    CreatePersonsPrepend(usize),
    ChangeRatio(String),
    DeletePersonById(usize),
    DeleteEverybody,
    SwapRandom,
    ReverseList,
    SortById,
    SortByName,
    SortByAge,
    SortByAddress,
    ToggleKeyed,
    Rendered(Instant),
}

pub struct Model {
    link: ComponentLink<Self>,
    persons: Vec<PersonType>,
    last_id: usize,
    keyed: bool,
    build_component_ratio: f64,
    delta_ref: NodeRef,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            persons: Vec::with_capacity(200),
            last_id: 0,
            keyed: true,
            build_component_ratio: 0.5,
            delta_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreatePersons(n) => {
                for _ in 0..n {
                    self.last_id += 1;
                    self.persons.push(PersonType::new_random(
                        self.last_id,
                        self.build_component_ratio,
                    ));
                }
                true
            }
            Msg::CreatePersonsPrepend(n) => {
                for _ in 0..n {
                    self.last_id += 1;
                    self.persons.insert(
                        0,
                        PersonType::new_random(self.last_id, self.build_component_ratio),
                    );
                }
                true
            }
            Msg::ChangeRatio(ratio) => {
                let ratio: f64 = ratio.parse().unwrap_or(0.5);
                if self.build_component_ratio.neq_assign(ratio) {
                    log::info!("Ratio changed: {}", ratio);
                    true
                } else {
                    false
                }
            }
            Msg::DeletePersonById(id) => {
                if let Some(idx) = self.persons.iter().position(|p| p.info().id == id) {
                    self.persons.remove(idx);
                    true
                } else {
                    false
                }
            }
            Msg::DeleteEverybody => {
                self.persons.clear();
                true
            }
            Msg::SwapRandom => {
                let (a, b) = random::choose_two_distinct_mut(&mut self.persons).unwrap();
                log::info!("Swapping {} and {}.", a.info().id, b.info().id);
                std::mem::swap(a, b);
                true
            }
            Msg::ReverseList => {
                self.persons.reverse();
                true
            }
            Msg::SortById => {
                self.persons
                    .sort_unstable_by(|a, b| a.info().id.cmp(&b.info().id));
                true
            }
            Msg::SortByName => {
                self.persons
                    .sort_unstable_by(|a, b| a.info().name.cmp(&b.info().name));
                true
            }
            Msg::SortByAge => {
                self.persons.sort_by_key(|p| p.info().age);
                true
            }
            Msg::SortByAddress => {
                self.persons
                    .sort_unstable_by(|a, b| a.info().address.cmp(&b.info().address));
                true
            }
            Msg::ToggleKeyed => {
                self.keyed = !self.keyed;
                true
            }
            Msg::Rendered(time_before) => {
                let time_after = Instant::now();
                let elapsed_max = time_after - time_before;
                log::info!("Rendering started {} ms ago.", elapsed_max.as_millis());
                if let Some(input) = self.delta_ref.cast::<HtmlElement>() {
                    let delta_text =
                        format!("The last rendering took {} ms", elapsed_max.as_millis());
                    input.set_inner_text(&delta_text);
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.link.send_message(Msg::Rendered(Instant::now()));

        html! {
            <div class="container">
                <div class="row">
                    <p class="h2" ref=self.delta_ref.clone()/>
                    <hr />
                </div>
                { self.action_view() }
                { self.info_view() }
            </div>
        }
    }
}

impl Model {
    fn action_view(&self) -> Html {
        html! {
            <>
                { self.button_view() }
                <div class="row">
                    <div class="col">
                        <p class="h5">
                            { "Person type ratio (0=only tags <= ratio <= 1=only components): " }
                            { self.build_component_ratio }
                        </p>
                        <input name="ratio" type="range" class="form-control-range" min="0.0" max="1.0" step="any"
                            value=self.build_component_ratio
                            oninput=self.link.callback(|e: InputData| Msg::ChangeRatio(e.value))
                        />
                    </div>
                </div>
            </>
        }
    }
    fn button_view(&self) -> Html {
        html! {
            <>
                <div class="row">
                    <div class="col">
                        <button class="btn_size alert alert-danger" onclick=self.link.callback(|_| Msg::DeleteEverybody)>
                            { "Delete everybody" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick=self.link.callback(|_| Msg::CreatePersons(1))>
                            { "Create 1" }
                    </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick=self.link.callback(|_| Msg::CreatePersons(5))>
                            { "Create 5" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick=self.link.callback(|_| Msg::CreatePersons(100))>
                            { "Create 100" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick=self.link.callback(|_| Msg::CreatePersons(500))>
                            { "Create 500" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick=self.link.callback(|_| Msg::CreatePersonsPrepend(1))>
                            { "Prepend 1" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-success" onclick=self.link.callback(|_| Msg::CreatePersonsPrepend(5))>
                            { "Prepend 5" }
                        </button>
                    </div>
                </div>
                <div class="row">
                    <div class="col">
                        <button class="btn_size alert alert-warning" onclick=self.link.callback(|_| Msg::ToggleKeyed)>
                            { if self.keyed { "Disable keys" } else { "Enable keys" } }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick=self.link.callback(|_| Msg::SwapRandom)>
                            { "Swap random" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick=self.link.callback(|_| Msg::ReverseList)>
                            { "Reverse list" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick=self.link.callback(|_| Msg::SortById)>
                            { "Sort by id" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick=self.link.callback(|_| Msg::SortByName)>
                            { "Sort by name" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick=self.link.callback(|_| Msg::SortByAge)>
                            { "Sort by age" }
                        </button>
                    </div>
                    <div class="col">
                        <button class="btn_size alert alert-info" onclick=self.link.callback(|_| Msg::SortByAddress)>
                            { "Sort by address" }
                        </button>
                    </div>
                </div>
            </>
        }
    }
    fn info_view(&self) -> Html {
        let ids = if self.persons.len() < 20 {
            self.persons
                .iter()
                .map(|p| p.info().id.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            String::from("<too many>")
        };
        html! {
            <div>
                <p class="h5">{ "Number of persons: " }{ self.persons.len() }</p>
                <p class="h5">{ "Ids: " }{ ids }</p>
                <hr />
                <div class="persons">
                    { for self.persons.iter().map(|p| p.render(self.keyed)) }
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
