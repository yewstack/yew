use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD, Engine};
use gloo::file::{callbacks::FileReader, File};
use gloo_console::debug;
use web_sys::{FormData, HtmlFormElement, File as RawFile};
use yew::prelude::*;

pub struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(FileDetails),
    Submit(SubmitEvent),
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
                let name = file.name.clone();
                self.files.push(file);
                self.readers.remove(&name);
            }
            Msg::Submit(event) => {
            	debug!(event.clone());
            	event.prevent_default();
                let form: HtmlFormElement = event.target_unchecked_into();
                let form_data = FormData::new_with_form(&form).expect("form data");
                let image_file = File::from(RawFile::from(form_data.get("file")));
                let link = ctx.link().clone();
                let name = image_file.name().clone();
                let file_type = image_file.raw_mime_type();
                let task = {
                    gloo::file::callbacks::read_as_bytes(&image_file, move |res| {
                        link.send_message(Msg::Loaded(FileDetails{
                            name,
                            file_type,
                            data: res.expect("failed to read file"),                        	
                        }));
                  	})
                };
                self.readers.insert(image_file.name(), task);
                
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <form onsubmit={ctx.link().callback(Msg::Submit)}>
                	<input name="alt" />
	                <input
	                    id="file"
	                    name="file"
	                    type="file"
	                    accept="image/*"
	                    multiple={false}
	                />
	                <input type="submit"/>
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
                    <img src={src} />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}