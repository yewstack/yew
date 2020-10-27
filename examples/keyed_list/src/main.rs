use instant::Instant;
use person::PersonType;
use yew::prelude::*;
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
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.link.send_message(Msg::Rendered(Instant::now()));

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
            <>
                <div class="buttons">
                    <button onclick=self.link.callback(|_| Msg::DeleteEverybody)>
                        { "Delete everybody" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersons(1))>
                        { "Create 1" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersons(5))>
                        { "Create 5" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersons(100))>
                        { "Create 100" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersons(500))>
                        { "Create 500" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersonsPrepend(1))>
                        { "Prepend 1" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::CreatePersonsPrepend(5))>
                        { "Prepend 5" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SwapRandom)>
                        { "Swap random" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::ReverseList)>
                        { "Reverse list" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortById)>
                        { "Sort by id" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortByName)>
                        { "Sort by name" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortByAge)>
                        { "Sort by age" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::SortByAddress)>
                        { "Sort by address" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::ToggleKeyed)>
                        { if self.keyed { "Disable keys" } else { "Enable keys" } }
                    </button>
                </div>
                <div class="ratio">
                    <label for="ratio">{ "Person type ratio (0=only tags <= ratio <= 1=only components): " }</label>
                    <input
                        class="input" type="text" id="ratio"
                        value=self.build_component_ratio.to_string()
                        oninput=self.link.callback(|e: InputData| Msg::ChangeRatio(e.value))
                    />
                </div>
                <p>{ "Number of persons: " }{ self.persons.len() }</p>
                <p>{ "Ids: " }{ ids }</p>
                <hr />
                <div class="persons">
                    { for self.persons.iter().map(|p| p.render(self.keyed)) }
                </div>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
