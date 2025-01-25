extern crate base64;
use std::collections::HashMap;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo::file::callbacks::FileReader;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

pub struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(FileDetails),
    Files(Option<web_sys::FileList>),
}

pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.readers.remove(&file.name);
                self.files.push(file);
                true
            }
            Msg::Files(files) => {
                for file in gloo::file::FileList::from(files.expect("files")).iter() {
                    let link = ctx.link().clone();
                    let name = file.name().clone();
                    let file_type = file.raw_mime_type();

                    let task = {
                        gloo::file::callbacks::read_as_bytes(file, move |res| {
                            link.send_message(Msg::Loaded(FileDetails {
                                data: res.expect("failed to read file"),
                                file_type,
                                name,
                            }))
                        })
                    };
                    self.readers.insert(file.name(), task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let noop_drag = Callback::from(|e: DragEvent| {
            e.prevent_default();
        });

        html! {
            <div id="wrapper">
                <p id="title">{ "Upload Your Files To The Cloud" }</p>
                <label for="file-upload">
                    <div
                        id="drop-container"
                        ondrop={ctx.link().callback(|event: DragEvent| {
                            event.prevent_default();
                            Msg::Files(event.data_transfer().unwrap().files())
                        })}
                        ondragover={&noop_drag}
                        ondragenter={&noop_drag}
                    >
                        <i class="fa fa-cloud-upload"></i>
                        <p>{"Drop your images here or click to select"}</p>
                    </div>
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*,video/*"
                    multiple={true}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::Files(input.files())
                    })}
                />
                <div id="preview-area">
                    { for self.files.iter().map(Self::view_file) }
                </div>
            </div>
        }
    }
}

impl App {
    fn view_file(file: &FileDetails) -> Html {
        let file_type = file.file_type.to_string();
        let src = format!("data:{};base64,{}", file_type, STANDARD.encode(&file.data));
        html! {
            <div class="preview-tile">
                <p class="preview-name">{ &file.name }</p>
                <div class="preview-media">
                    if file.file_type.contains("image") {
                        <img src={src} />
                    } else if file.file_type.contains("video") {
                        <video controls={true}>
                            <source src={src} type={ file_type }/>
                        </video>
                    }
                </div>
            </div>
        }
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
