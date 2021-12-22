use js_sys::Math;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, Node};

const ADJECTIVES_LEN: usize = 25;
const ADJECTIVES_LEN_F64: f64 = ADJECTIVES_LEN as f64;
const ADJECTIVES: [&str; ADJECTIVES_LEN] = [
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

const COLOURS_LEN: usize = 11;
const COLOURS_LEN_F64: f64 = COLOURS_LEN as f64;
const COLOURS: [&str; COLOURS_LEN] = [
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

const NOUNS_LEN: usize = 13;
const NOUNS_LEN_F64: f64 = NOUNS_LEN as f64;
const NOUNS: [&str; NOUNS_LEN] = [
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

fn random(max: f64) -> usize {
    ((Math::random() * 1000.0) % max) as usize
}

struct Row {
    id: usize,
    label: String,
    el: Element,
    label_node: Node,
}

const ROW_TEMPLATE: &str = "<td class='col-md-1'></td><td class='col-md-4'><a class='lbl'></a></td><td class='col-md-1'><a class='remove'><span class='remove glyphicon glyphicon-remove' aria-hidden='true'></span></a></td><td class='col-md-6'></td>";

struct Main {
    document: Document,
    data: Vec<Row>,
    row_template: Node,
    tbody: Node,
    last_id: usize,
    selected: Option<Element>,
}

fn get_parent_id(el: Element) -> Option<usize> {
    let mut current = Some(el);
    while let Some(e) = current {
        if e.tag_name() == "TR" {
            return match e.get_attribute("data-id") {
                Some(id) => Some(id.parse::<usize>().unwrap()),
                None => None,
            };
        }
        current = e.parent_element();
    }
    None
}

impl Main {
    fn run(&mut self) -> Result<(), JsValue> {
        self.clear();
        self.append_rows(1000)
    }

    fn add(&mut self) -> Result<(), JsValue> {
        self.append_rows(1000)
    }

    fn update(&mut self) {
        let mut i = 0;
        let l = self.data.len();
        while i < l {
            let row = &mut self.data[i];
            row.label.push_str(" !!!");
            row.label_node.set_text_content(Some(row.label.as_str()));
            i += 10;
        }
    }

    fn unselect(&mut self) {
        if let Some(el) = self.selected.take() {
            el.set_class_name("");
        }
    }

    fn select(&mut self, id: usize) {
        self.unselect();
        for row in &self.data {
            if row.id == id {
                row.el.set_class_name("danger");
                self.selected = Some(row.el.clone());
                return;
            }
        }
    }

    fn delete(&mut self, id: usize) {
        let row = match self.data.iter().position(|row| row.id == id) {
            Some(i) => self.data.remove(i),
            None => return,
        };
        row.el.remove();
    }

    fn clear(&mut self) {
        self.data = Vec::new();
        self.tbody.set_text_content(None);
        self.unselect();
    }

    fn run_lots(&mut self) -> Result<(), JsValue> {
        self.clear();
        self.append_rows(10000)
    }

    fn swap_rows(&mut self) -> Result<(), JsValue> {
        if self.data.len() <= 998 {
            return Ok(());
        }
        let row1 = &self.data[1];
        let row998 = &self.data[998];
        let a = &row1.el;
        let b = a.next_sibling().unwrap();
        let c = &row998.el;
        let d = c.next_sibling().unwrap();
        self.tbody.insert_before(&c, Some(&b))?;
        self.tbody.insert_before(&a, Some(&d))?;
        self.data.swap(1, 998);
        Ok(())
    }

    fn append_rows(&mut self, count: usize) -> Result<(), JsValue> {
        self.data.reserve(count);
        for i in 0..count {
            let id = self.last_id + i + 1;

            let adjective = ADJECTIVES[random(ADJECTIVES_LEN_F64)];
            let colour = COLOURS[random(COLOURS_LEN_F64)];
            let noun = NOUNS[random(NOUNS_LEN_F64)];
            let capacity = adjective.len() + colour.len() + noun.len() + 2;
            let mut label = String::with_capacity(capacity);
            label.push_str(adjective);
            label.push(' ');
            label.push_str(colour);
            label.push(' ');
            label.push_str(noun);

            let node = self.row_template.clone_node_with_deep(true)?;
            let id_node = node.first_child().unwrap();
            let label_node = id_node.next_sibling().unwrap().first_child().unwrap();
            let id_string = id.to_string();
            let id_str = id_string.as_str();
            id_node.set_text_content(Some(id_str));
            label_node.set_text_content(Some(label.as_str()));

            let el = JsCast::unchecked_into::<Element>(node);
            el.set_attribute("data-id", id_str)?;
            let row = Row {
                id,
                label,
                el,
                label_node,
            };

            self.tbody.append_child(&row.el)?;
            self.data.push(row);
        }
        self.last_id += count;
        Ok(())
    }

}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let row_template = document.create_element("tr")?;
    row_template.set_inner_html(ROW_TEMPLATE);

    let tbody = document.get_element_by_id("tbody").unwrap();

    let main = RefCell::new(Rc::new(Main {
        document,
        data: Vec::new(),
        row_template: row_template.into(),
        tbody: tbody.into(),
        last_id: 0,
        selected: None,
    }));

    let main2 = main.clone();
    let onclick = Closure::wrap(Box::new(move |e: Event| {
        let target = match e.target() {
            Some(target) => target,
            None => return,
        };
        let el = JsCast::unchecked_ref::<Element>(&target);
        let mut m = main2.borrow_mut();
        let main = match Rc::get_mut(&mut m) {
            Some(main) => main,
            None => return,
        };

        match el.id().as_str() {
            "add" => {
                e.prevent_default();
                main.add().unwrap();
            }
            "run" => {
                e.prevent_default();
                main.run().unwrap();
            }
            "update" => {
                e.prevent_default();
                main.update();
            }
            "runlots" => {
                e.prevent_default();
                main.run_lots().unwrap();
            }
            "clear" => {
                e.prevent_default();
                main.clear();
            }
            "swaprows" => {
                e.prevent_default();
                main.swap_rows().unwrap();
            }
            _ => {
                let class_list = el.class_list();
                if class_list.contains("remove") {
                    e.prevent_default();
                    let parent_id = match get_parent_id(el.clone()) {
                        Some(id) => id,
                        None => return,
                    };
                    main.delete(parent_id);
                } else if class_list.contains("lbl") {
                    e.prevent_default();
                    let parent_id = match get_parent_id(el.clone()) {
                        Some(id) => id,
                        None => return,
                    };
                    main.select(parent_id);
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    if let Ok(m) = &(main.try_borrow()) {
        let main_el = m.document.get_element_by_id("main").unwrap();
        main_el.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())?;
        onclick.forget();
    }

    Ok(())
}
