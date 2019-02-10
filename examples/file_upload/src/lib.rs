use yew::{html, ChangeData, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};

pub struct Model {
    link: ComponentLink<Model>,
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    files: Vec<FileData>,
}

pub enum Msg {
    Loaded(FileData),
    Files(Vec<File>),
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(file) => {
                self.files.push(file);
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let callback = self.link.send_back(Msg::Loaded);
                    let task = self.reader.read_file(file, callback);
                    self.tasks.push(task);
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input type="file", multiple=true, onchange=|value| {
                        let mut result = Vec::new();
                        if let ChangeData::Files(files) = value {
                            result.extend(files);
                        }
                        Msg::Files(result)
                    },/>
                <ul>
                    { for self.files.iter().map(|f| self.view_file(f)) }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_file(&self, file: &FileData) -> Html<Self> {
        html! {
            <li>{ format!("file: {}, size: {}", file.name, file.content.len()) }</li>
        }
    }
}
