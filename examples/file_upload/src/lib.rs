#![recursion_limit = "256"]

use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::{html, ChangeData, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    link: ComponentLink<Model>,
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    files: Vec<String>,
    by_chunks: bool,
}

type Chunks = bool;

pub enum Msg {
    Loaded(FileData),
    Chunk(FileChunk),
    Files(Vec<File>, Chunks),
    ToggleByChunks,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            reader: ReaderService::new(),
            link,
            tasks: vec![],
            files: vec![],
            by_chunks: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(file) => {
                let info = format!("file: {:?}", file);
                self.files.push(info);
            }
            Msg::Chunk(chunk) => {
                let info = format!("chunk: {:?}", chunk);
                self.files.push(info);
            }
            Msg::Files(files, chunks) => {
                for file in files.into_iter() {
                    let task = {
                        if chunks {
                            let callback = self.link.send_back(Msg::Chunk);
                            self.reader.read_file_by_chunks(file, callback, 10)
                        } else {
                            let callback = self.link.send_back(Msg::Loaded);
                            self.reader.read_file(file, callback)
                        }
                    };
                    self.tasks.push(task);
                }
            }
            Msg::ToggleByChunks => {
                self.by_chunks = !self.by_chunks;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let flag = self.by_chunks;
        html! {
            <div>
                <div>
                    <input type="file" multiple=true onchange=|value| {
                            let mut result = Vec::new();
                            if let ChangeData::Files(files) = value {
                                result.extend(files);
                            }
                            Msg::Files(result, flag)
                        }/>
                </div>
                <div>
                    <label>{ "By chunks" }</label>
                    <input type="checkbox" checked=flag onclick=|_| Msg::ToggleByChunks />
                </div>
                <ul>
                    { for self.files.iter().map(|f| self.view_file(f)) }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_file(&self, data: &str) -> Html<Self> {
        html! {
            <li>{ data }</li>
        }
    }
}
