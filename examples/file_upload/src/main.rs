use std::collections::HashMap;

use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};

type FileName = String;
type Chunks = bool;

pub enum Msg {
    Loaded((FileName, FileData)),
    Chunk((FileName, Option<FileChunk>)),
    Files(Vec<File>, Chunks),
    ToggleByChunks,
}

pub struct Model {
    link: ComponentLink<Model>,
    tasks: HashMap<FileName, ReaderTask>,
    files: Vec<String>,
    by_chunks: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tasks: HashMap::default(),
            files: vec![],
            by_chunks: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded((file_name, file)) => {
                let info = format!("file: {:?}", file);
                self.files.push(info);
                self.tasks.remove(&file_name);
                true
            }
            Msg::Chunk((file_name, Some(chunk))) => {
                let info = format!("chunk: {:?}", chunk);
                self.files.push(info);
                if let FileChunk::Finished = chunk {
                    self.tasks.remove(&file_name);
                }
                true
            }
            Msg::Files(files, chunks) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        if chunks {
                            let callback = self
                                .link
                                .callback(move |chunk| Msg::Chunk((file_name.clone(), chunk)));
                            ReaderService::read_file_by_chunks(file, callback, 10).unwrap()
                        } else {
                            let callback = self
                                .link
                                .callback(move |data| Msg::Loaded((file_name.clone(), data)));
                            ReaderService::read_file(file, callback).unwrap()
                        }
                    };
                    self.tasks.insert(file_name, task);
                }
                true
            }
            Msg::ToggleByChunks => {
                self.by_chunks = !self.by_chunks;
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let flag = self.by_chunks;
        html! {
            <div>
                <div>
                    <p>{ "Choose a file to upload to see the uploaded bytes" }</p>
                    <input type="file" multiple=true onchange=self.link.callback(move |value| {
                            let mut result = Vec::new();
                            if let ChangeData::Files(files) = value {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .map(|v| File::from(v.unwrap()));
                                result.extend(files);
                            }
                            Msg::Files(result, flag)
                        })
                    />
                </div>
                <div>
                    <label>{ "By chunks" }</label>
                    <input type="checkbox" checked=flag onclick=self.link.callback(|_| Msg::ToggleByChunks) />
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
