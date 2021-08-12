use crate::Client;
use yew::{
    classes, html,
    web_sys::{Event, HtmlInputElement, HtmlTextAreaElement},
    Callback, Component, ComponentLink, Html, Properties, ShouldRender, TargetCast,
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

        let update_name = |f: fn(String) -> Msg| {
            link.callback(move |e: Event| {
                let input: HtmlInputElement = e.target_unchecked_into();
                f(input.value())
            })
        };

        let update_desc = link.callback(|e: Event| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::UpdateDescription(textarea.value())
        });

        html! {
            <>
                <div class="names">
                    <input
                        class={classes!("new-client", "firstname")}
                        placeholder="First name"
                        onchange={update_name(Msg::UpdateFirstName)}
                    />
                    <input
                        class={classes!("new-client", "lastname")}
                        placeholder="Last name"
                        onchange={update_name(Msg::UpdateLastName)}
                    />
                    <textarea
                        class={classes!("new-client", "description")}
                        placeholder="Description"
                        onchange={update_desc}
                    />
                </div>

                <button
                    disabled={client.first_name.is_empty() || client.last_name.is_empty()}
                    onclick={link.callback(|_| Msg::Add)}
                >
                    { "Add New" }
                </button>
                <button onclick={link.callback(|_| Msg::Abort)}>
                    { "Go Back" }
                </button>
            </>
        }
    }
}
