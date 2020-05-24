#![recursion_limit = "1024"]

use instant::Instant;
use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Model {
    link: ComponentLink<Self>,
    persons: Vec<PersonType>,
    last_id: usize,
    keyed: bool,
    build_component_ratio: f64,
}

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

enum PersonType {
    Basic(PersonInfo),
    Component(PersonInfo),
}

#[derive(PartialEq, Debug, Clone)]
struct PersonInfo {
    id: usize,
    name: Rc<String>,
    address: Rc<String>,
    age: usize,
}

struct PersonComponent {
    info: PersonInfo,
}

#[derive(PartialEq, Clone, Properties)]
struct PersonProps {
    info: PersonInfo,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), link: ComponentLink<Self>) -> Self {
        Model {
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
                let idx_a = rand::thread_rng().gen::<usize>() % self.persons.len();
                let idx_b = rand::thread_rng().gen::<usize>() % self.persons.len();
                let id_a = self.persons.get(idx_a).unwrap().info().id;
                let id_b = self.persons.get(idx_b).unwrap().info().id;
                log::info!("Swapping {} and {}.", id_a, id_b);
                self.persons.swap(idx_a, idx_b);
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
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
                        value=self.build_component_ratio
                        oninput=self.link.callback(|e: InputData| Msg::ChangeRatio(e.value))
                        />
                </div>
                <p>{ "Number of persons: " }{ self.persons.len() }</p>
                <p>{ "Ids: " }{ ids }</p>
                <hr />
                <div class="persons">
                    { for self.persons.iter().map(|p| match p {
                        PersonType::Basic(info) if self.keyed => {
                            html! {
                                <div class="basic-person" key=info.id.to_string() id=info.id.to_string()>
                                    { info.render() }
                                </div>
                            }
                        },
                        PersonType::Basic(info) => {
                            html! {
                                <div class="basic-person" id=info.id.to_string()>
                                    { info.render() }
                                </div>
                            }
                        },
                        PersonType::Component(info) if self.keyed => html! { <PersonComponent info=info key=info.id.to_string() /> },
                        PersonType::Component(info) => html! { <PersonComponent info=info /> },
                    })}
                </div>
            </>
        }
    }
}

impl Component for PersonComponent {
    type Message = ();
    type Properties = PersonProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // log::debug!("Created person {}", props.info.id);
        PersonComponent { info: props.info }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // log::debug!("Changed person: {} -> {}", self.info.id, props.info.id);
        self.info.neq_assign(props.info)
    }

    fn view(&self) -> Html {
        html! {
            <div class="component-person" id=self.info.id.to_string()>
                { self.info.render() }
            </div>
        }
    }
}

impl PersonType {
    fn info(&self) -> &PersonInfo {
        match self {
            PersonType::Basic(info) => info,
            PersonType::Component(info) => info,
        }
    }

    fn new_random(id: usize, ratio: f64) -> Self {
        let info = PersonInfo::new_random(id);
        if (rand::thread_rng().gen::<f64>() % 1.0) > ratio {
            PersonType::Basic(info)
        } else {
            PersonType::Component(info)
        }
    }
}

impl PersonInfo {
    fn new_random(id: usize) -> Self {
        PersonInfo {
            id,
            name: Rc::new(PersonInfo::gen_name()),
            age: PersonInfo::gen_age(),
            address: Rc::new(PersonInfo::gen_address()),
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

    fn gen_number(min: usize, max: usize) -> usize {
        let len: usize = rand::thread_rng().gen();
        len % (max - min) + min
    }

    fn gen_string(len: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..len)
            .map(|_| rng.sample(rand::distributions::Alphanumeric))
            .collect()
    }

    fn gen_words(n_words: usize, min_len: usize, max_len: usize) -> Vec<String> {
        (0..n_words)
            .map(|_| PersonInfo::gen_string(PersonInfo::gen_number(min_len, max_len)))
            .collect()
    }

    fn gen_name() -> String {
        PersonInfo::gen_words(2, 4, 15).join(" ")
    }

    fn gen_age() -> usize {
        PersonInfo::gen_number(7, 77)
    }

    fn gen_address() -> String {
        let n_words = PersonInfo::gen_number(3, 6);
        PersonInfo::gen_words(n_words, 5, 12).join(" ")
    }
}
