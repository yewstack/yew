use std::collections::HashMap;

use web_sys::{Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html};

use gloo_file::callbacks::FileReader;
use gloo_file::File;

type Chunks = bool;

pub enum Msg {
    Loaded(String, String),
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>, Chunks),
    ToggleReadBytes,
}

pub struct Model {
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
    read_bytes: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
            read_bytes: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
                self.readers.remove(&file_name);
                true
            }
            Msg::LoadedBytes(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                self.files.push(info);
                self.readers.remove(&file_name);
                true
            }
            Msg::Files(files, bytes) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        if bytes {
                            gloo_file::callbacks::read_as_bytes(&file, move |res| {
                                link.send_message(Msg::LoadedBytes(
                                    file_name,
                                    res.expect("failed to read file"),
                                ))
                            })
                        } else {
                            gloo_file::callbacks::read_as_text(&file, move |res| {
                                link.send_message(Msg::Loaded(
                                    file_name,
                                    res.unwrap_or_else(|e| e.to_string()),
                                ))
                            })
                        }
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::ToggleReadBytes => {
                self.read_bytes = !self.read_bytes;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flag = self.read_bytes;
        html! {
            <div>
                <div>
                    <p>{ "Choose a file to upload to see the uploaded bytes" }</p>
                    <input type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
                            let mut result = Vec::new();
                            let input: HtmlInputElement = e.target_unchecked_into();

                            if let Some(files) = input.files() {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .map(|v| web_sys::File::from(v.unwrap()))
                                    .map(File::from);
                                result.extend(files);
                            }
                            Msg::Files(result, flag)
                        })}
                    />
                </div>
                <div>
                    <label>{ "Read bytes" }</label>
                    <input type="checkbox" checked={flag} onclick={ctx.link().callback(|_| Msg::ToggleReadBytes)} />
                </div>
                <ul>
                    { for self.files.iter().map(|f| Self::view_file(f)) }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_file(data: &str) -> Html {
        html! {
            <li>{ data }</li>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
