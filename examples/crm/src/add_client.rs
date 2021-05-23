use crate::Client;
use yew::{
    classes, html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender,
};

#[derive(Debug)]
pub enum Msg {
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateDescription(String),
    Add,
    Abort,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_add: Callback<Client>,
    pub on_abort: Callback<()>,
}

pub struct AddClientForm {
    props: Props,
    link: ComponentLink<Self>,
    client: Client,
}
impl Component for AddClientForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            client: Client::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let client = &mut self.client;
        match msg {
            Msg::UpdateFirstName(value) => {
                client.first_name = value;
                true
            }
            Msg::UpdateLastName(value) => {
                client.last_name = value;
                true
            }
            Msg::UpdateDescription(value) => {
                client.description = value;
                true
            }
            Msg::Add => {
                self.props.on_add.emit(std::mem::take(client));
                true
            }
            Msg::Abort => {
                self.props.on_abort.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let Self { link, client, .. } = self;
        html! {
            <>
                <div class="names">
                    <input
                        class=classes!("new-client", "firstname")
                        placeholder="First name"
                        value=&client.first_name
                        oninput=link.callback(|e: InputData| Msg::UpdateFirstName(e.value))
                    />
                    <input
                        class=classes!("new-client", "lastname")
                        placeholder="Last name"
                        value=&client.last_name
                        oninput=link.callback(|e: InputData| Msg::UpdateLastName(e.value))
                    />
                    <textarea
                        class=classes!("new-client", "description")
                        placeholder="Description"
                        value=&client.description
                        oninput=link.callback(|e: InputData| Msg::UpdateDescription(e.value))
                    />
                </div>

                <button
                    disabled=client.first_name.is_empty() || client.last_name.is_empty()
                    onclick=link.callback(|_| Msg::Add)
                >
                    { "Add New" }
                </button>
                <button onclick=link.callback(|_| Msg::Abort)>
                    { "Go Back" }
                </button>
            </>
        }
    }
}
