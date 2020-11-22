use crate::Client;
use yew::{html, Callback, Component, Context, Html, InputData, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateDescription(String),
    Add,
    Abort,
}

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub on_add: Callback<Client>,
    pub on_abort: Callback<()>,
}

pub struct AddClientForm {
    client: Client,
}

impl Component for AddClientForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            client: Client::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
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
                ctx.props.on_add.emit(std::mem::take(client));
                true
            }
            Msg::Abort => {
                ctx.props.on_abort.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="names">
                    <input
                        class=("new-client", "firstname")
                        placeholder="First name"
                        value=&self.client.first_name
                        oninput=ctx.callback(|e: InputData| Msg::UpdateFirstName(e.value))
                    />
                    <input
                        class=("new-client", "lastname")
                        placeholder="Last name"
                        value=&self.client.last_name
                        oninput=ctx.callback(|e: InputData| Msg::UpdateLastName(e.value))
                    />
                    <textarea
                        class=("new-client", "description")
                        placeholder="Description"
                        value=&self.client.description
                        oninput=ctx.callback(|e: InputData| Msg::UpdateDescription(e.value))
                    />
                </div>

                <button
                    disabled=self.client.first_name.is_empty() || self.client.last_name.is_empty()
                    onclick=ctx.callback(|_| Msg::Add)
                >
                    { "Add New" }
                </button>
                <button onclick=ctx.callback(|_| Msg::Abort)>
                    { "Go Back" }
                </button>
            </>
        }
    }
}
