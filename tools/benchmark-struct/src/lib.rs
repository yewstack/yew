use rand::prelude::*;
use std::cmp::min;
use wasm_bindgen::prelude::*;
use web_sys::window;
use yew::prelude::*;

static ADJECTIVES: &[&str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

#[derive(Clone, PartialEq)]
struct RowData {
    id: usize,
    label: String,
}

impl RowData {
    fn new(id: usize, rng: &mut SmallRng) -> Self {
        let adjective = *ADJECTIVES.choose(rng).unwrap();
        let colour = *COLOURS.choose(rng).unwrap();
        let noun = *NOUNS.choose(rng).unwrap();

        let label = [adjective, colour, noun].join(" ");

        Self { id, label }
    }
}

struct App {
    rows: Vec<RowData>,
    next_id: usize,
    selected_id: Option<usize>,
    rng: SmallRng,
    on_select: Callback<usize>,
    on_remove: Callback<usize>,
}

enum Msg {
    Run(usize),
    Add(usize),
    Update(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App {
            rows: Vec::new(),
            next_id: 1,
            selected_id: None,
            rng: SmallRng::from_entropy(),
            on_select: ctx.link().callback(Msg::Select),
            on_remove: ctx.link().callback(Msg::Remove),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Run(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                let update_amount = min(amount, self.rows.len());
                for index in 0..update_amount {
                    self.rows[index] = RowData::new(next_id + index, rng);
                }
                self.rows.extend(
                    (update_amount..amount).map(|index| RowData::new(next_id + index, rng)),
                );
                self.next_id += amount;
            }
            Msg::Add(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                self.rows
                    .extend((0..amount).map(|index| RowData::new(next_id + index, rng)));
                self.next_id += amount;
            }
            Msg::Update(step) => {
                for index in (0..self.rows.len()).step_by(step) {
                    self.rows[index].label += " !!!";
                }
            }
            Msg::Clear => {
                self.rows.clear();
            }
            Msg::Swap => {
                if self.rows.len() > 998 {
                    self.rows.swap(1, 998);
                }
            }
            Msg::Remove(id) => {
                if let Some(index) = self.rows.iter().position(|row| row.id == id) {
                    self.rows.remove(index);
                }
            }
            Msg::Select(id) => {
                self.selected_id = Some(id);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rows: Html = self
            .rows
            .iter()
            .map(|row| {
                html! {
                    <Row
                        key={row.id}
                        data={row.clone()}
                        selected={self.selected_id == Some(row.id)}
                        on_select={self.on_select.clone()}
                        on_remove={self.on_remove.clone()}
                    />
                }
            })
            .collect();

        html! {
            <div class="container">
                <Jumbotron
                    on_run={ctx.link().callback(Msg::Run)}
                    on_add={ctx.link().callback(Msg::Add)}
                    on_update={ctx.link().callback(Msg::Update)}
                    on_clear={ctx.link().callback(|_| Msg::Clear)}
                    on_swap={ctx.link().callback(|_| Msg::Swap)}
                />
                <table class="table table-hover table-striped test-data">
                    <tbody id="tbody">
                        { rows }
                    </tbody>
                </table>
                <span class="preloadicon glyphicon glyphicon-remove" aria-hidden="true"></span>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct JumbotronProps {
    pub on_run: Callback<usize>,
    pub on_add: Callback<usize>,
    pub on_update: Callback<usize>,
    pub on_clear: Callback<()>,
    pub on_swap: Callback<()>,
}

pub struct Jumbotron {}

impl Component for Jumbotron {
    type Properties = JumbotronProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="jumbotron">
                <div class="row">
                    <div class="col-md-6">
                        <h1>{ "Yew-Hooks" }</h1>
                    </div>
                    <div class="col-md-6">
                        <div class="row">
                            <div class="col-sm-6 smallpad">
                                <button type="button" id="run" class="btn btn-primary btn-block" onclick={ctx.props().on_run.reform(|_| 1_000)}>{ "Create 1,000 rows" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_run.reform(|_| 10_000)} id="runlots">{ "Create 10,000 rows" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_add.reform(|_| 1_000)} id="add">{ "Append 1,000 rows" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_update.reform(|_| 10)} id="update">{ "Update every 10th row" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_clear.reform(|_| ())} id="clear">{ "Clear" }</button>
                            </div>
                            <div class="col-sm-6 smallpad">
                                <button type="button" class="btn btn-primary btn-block" onclick={ctx.props().on_swap.reform(|_| ())} id="swaprows">{ "Swap Rows" }</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct RowProps {
    on_select: Callback<usize>,
    on_remove: Callback<usize>,
    selected: bool,
    data: RowData,
}

struct Row {
    on_select: Callback<MouseEvent>,
    on_remove: Callback<MouseEvent>,
}

impl Component for Row {
    type Properties = RowProps;
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().data.id;
        Self {
            on_select: ctx.props().on_select.reform(move |_| id),
            on_remove: ctx.props().on_remove.reform(move |_| id),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let id = ctx.props().data.id;
        self.on_select = ctx.props().on_select.reform(move |_| id);
        self.on_remove = ctx.props().on_remove.reform(move |_| id);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <tr class={if ctx.props().selected { "danger" } else  { "" }}>
                <td class="col-md-1">{ ctx.props().data.id }</td>
                <td class="col-md-4" onclick={self.on_select.clone()}>
                    <a class="lbl">{ ctx.props().data.label.clone() }</a>
                </td>
                <td class="col-md-1">
                    <a class="remove" onclick={self.on_remove.clone()}>
                        <span class="glyphicon glyphicon-remove remove" aria-hidden="true"></span>
                    </a>
                </td>
                <td class="col-md-6"></td>
            </tr>
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = window().unwrap().document().unwrap();
    let mount_el = document.query_selector("#main").unwrap().unwrap();
    yew::start_app_in_element::<App>(mount_el);
}
