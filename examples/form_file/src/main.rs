use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD, Engine};
use gloo::file::{callbacks::FileReader, File, FileList};
use gloo_console::debug;
use web_sys::{File as RawFile, FormData, HtmlFormElement, HtmlInputElement};
use yew::prelude::*;

pub struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
    alt_text: String,
}

pub enum Msg {
    Loaded(String, Vec<u8>),
    Submit(SubmitEvent),
    File(FileList),
}

pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
    button: NodeRef,
    file_data: Vec<u8>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
            button: NodeRef::default(),
            file_data: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(name, data) => {
                let submit = self.button.cast::<HtmlInputElement>().expect("button");
                self.file_data = data;
                submit.set_disabled(false);
                self.readers.remove(&name);
            }
            Msg::File(files) => {
                let submit = self.button.cast::<HtmlInputElement>().expect("button");
                submit.set_disabled(true);

                let file = files[0].clone();
                let link = ctx.link().clone();
                let name = file.name().clone();
                let task = {
                    gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        link.send_message(Msg::Loaded(name, res.expect("failed to read file")));
                    })
                };
                self.readers.insert(file.name(), task);
            }
            Msg::Submit(event) => {
                debug!(event.clone());
                event.prevent_default();
                let form: HtmlFormElement = event.target_unchecked_into();
                let form_data = FormData::new_with_form(&form).expect("form data");
                let image_file = File::from(RawFile::from(form_data.get("file")));

                let alt_text = form_data.get("alt-text").as_string().unwrap();
                let name = image_file.name();
                let data = self.file_data.clone();

                let file_type = image_file.raw_mime_type();
                self.files.push(FileDetails {
                    alt_text,
                    name,
                    data,
                    file_type,
                });
                self.file_data = Vec::default();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <form onsubmit={ctx.link().callback(Msg::Submit)}>
                    <lable for="alt-text">{"Alt Text"}</lable>
                    <input id="alt-text" name="alt-text" />
                    <input
                        id="file"
                        name="file"
                        type="file"
                        accept="image/*"
                        multiple={false}
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::File(gloo::file::FileList::from(input.files().expect("file")))
                            })}
                    />
                    <input ref={&self.button} type="submit"/>
                </form>
                <div id="preview-area">
                    { for self.files.iter().map(Self::view_file) }
                </div>
            </div>
        }
    }
}

impl App {
    fn view_file(file: &FileDetails) -> Html {
        let src = format!(
            "data:{};base64,{}",
            file.file_type,
            STANDARD.encode(&file.data)
        );
        html! {
            <div class="preview-tile">
                <p class="preview-name">{ format!("{}", file.name) }</p>
                <div class="preview-media">
                    <img src={src} alt={file.alt_text.clone()}/>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
