use rand::prelude::*;
use std::cmp::min;
use std::ops::Deref;
use std::rc::Rc;
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

enum AppStateAction {
    Run(usize),
    Add(usize),
    Update(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
}

#[derive(Clone)]
struct AppState {
    next_id: usize,
    selected_id: Option<usize>,
    rows: Vec<RowData>,
    rng: SmallRng,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            next_id: 1,
            selected_id: None,
            rng: SmallRng::from_entropy(),
        }
    }
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = self.deref().clone();
        match action {
            AppStateAction::Run(amount) => {
                let rng = &mut new_state.rng;
                let next_id = new_state.next_id;
                let update_amount = min(amount, new_state.rows.len());
                for index in 0..update_amount {
                    new_state.rows[index] = RowData::new(next_id + index, rng);
                }
                new_state.rows.extend(
                    (update_amount..amount).map(|index| RowData::new(next_id + index, rng)),
                );
                new_state.next_id += amount;
            }
            AppStateAction::Add(amount) => {
                let rng = &mut new_state.rng;
                let next_id = new_state.next_id;
                new_state
                    .rows
                    .extend((0..amount).map(|index| RowData::new(next_id + index, rng)));
                new_state.next_id += amount;
            }
            AppStateAction::Update(step) => {
                for index in (0..new_state.rows.len()).step_by(step) {
                    new_state.rows[index].label += " !!!";
                }
            }
            AppStateAction::Clear => {
                new_state.rows.clear();
            }
            AppStateAction::Swap => {
                if new_state.rows.len() > 998 {
                    new_state.rows.swap(1, 998);
                }
            }
            AppStateAction::Remove(id) => {
                if let Some(index) = new_state.rows.iter().position(|row| row.id == id) {
                    new_state.rows.remove(index);
                }
            }
            AppStateAction::Select(id) => {
                new_state.selected_id = Some(id);
            }
        };

        new_state.into()
    }
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(AppState::default);

    let selected_id = state.deref().selected_id;
    let rows = state.deref().rows.clone();

    let on_run = {
        let state = state.clone();
        Callback::from(move |amount| state.dispatch(AppStateAction::Run(amount)))
    };
    let on_add = {
        let state = state.clone();
        Callback::from(move |amount| state.dispatch(AppStateAction::Add(amount)))
    };
    let on_update = {
        let state = state.clone();
        Callback::from(move |amount| state.dispatch(AppStateAction::Update(amount)))
    };
    let on_clear = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(AppStateAction::Clear))
    };
    let on_swap = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(AppStateAction::Swap))
    };
    let on_select = {
        let state = state.clone();
        Callback::from(move |id| state.dispatch(AppStateAction::Select(id)))
    };
    let on_remove = Callback::from(move |id| state.dispatch(AppStateAction::Remove(id)));

    let rows: Html = rows
        .into_iter()
        .map(move |row| {
            let id = row.id;
            html! {
                <Row
                    key={id}
                    data={row}
                    selected={selected_id == Some(id)}
                    on_select={on_select.reform(move |_| id)}
                    on_remove={on_remove.reform(move |_| id)}
                />
            }
        })
        .collect();

    html! {
        <div class="container">
            <Jumbotron
                {on_run}
                {on_add}
                {on_update}
                {on_clear}
                {on_swap}
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

#[derive(Properties, Clone, PartialEq)]
pub struct JumbotronProps {
    pub on_run: Callback<usize>,
    pub on_add: Callback<usize>,
    pub on_update: Callback<usize>,
    pub on_clear: Callback<()>,
    pub on_swap: Callback<()>,
}

#[function_component(Jumbotron)]
fn jumbotron(props: &JumbotronProps) -> Html {
    html! {
        <div class="jumbotron">
            <div class="row">
                <div class="col-md-6">
                    <h1>{ "Yew" }</h1>
                </div>
                <div class="col-md-6">
                    <div class="row">
                        <div class="col-sm-6 smallpad">
                            <button type="button" id="run" class="btn btn-primary btn-block" onclick={props.on_run.reform(|_| 1_000)}>{ "Create 1,000 rows" }</button>
                        </div>
                        <div class="col-sm-6 smallpad">
                            <button type="button" class="btn btn-primary btn-block" onclick={props.on_run.reform(|_| 10_000)} id="runlots">{ "Create 10,000 rows" }</button>
                        </div>
                        <div class="col-sm-6 smallpad">
                            <button type="button" class="btn btn-primary btn-block" onclick={props.on_add.reform(|_| 1_000)} id="add">{ "Append 1,000 rows" }</button>
                        </div>
                        <div class="col-sm-6 smallpad">
                            <button type="button" class="btn btn-primary btn-block" onclick={props.on_update.reform(|_| 10)} id="update">{ "Update every 10th row" }</button>
                        </div>
                        <div class="col-sm-6 smallpad">
                            <button type="button" class="btn btn-primary btn-block" onclick={props.on_clear.reform(|_| ())} id="clear">{ "Clear" }</button>
                        </div>
                        <div class="col-sm-6 smallpad">
                            <button type="button" class="btn btn-primary btn-block" onclick={props.on_swap.reform(|_| ())} id="swaprows">{ "Swap Rows" }</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct RowProps {
    on_select: Callback<MouseEvent>,
    on_remove: Callback<MouseEvent>,
    selected: bool,
    data: RowData,
}

#[function_component(Row)]
fn row(props: &RowProps) -> Html {
    html! {
        <tr class={if props.selected { "danger" } else  { "" }}>
            <td class="col-md-1">{ props.data.id }</td>
            <td class="col-md-4" onclick={props.on_select.clone()}>
                <a class="lbl">{ props.data.label.clone() }</a>
            </td>
            <td class="col-md-1">
                <a class="remove" onclick={props.on_remove.clone()}>
                    <span class="glyphicon glyphicon-remove remove" aria-hidden="true"></span>
                </a>
            </td>
            <td class="col-md-6"></td>
        </tr>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = window().unwrap().document().unwrap();
    let mount_el = document.query_selector("#main").unwrap().unwrap();
    yew::start_app_in_element::<App>(mount_el);
}
