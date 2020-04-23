use crate::markdown::render_markdown;
use yew::{
    format::{Nothing, Text},
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    virtual_dom::VNode,
};

pub struct MarkdownWindow {
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    markdown: Option<String>,
    props: MdProps,
    link: ComponentLink<Self>,
}

#[derive(Properties, Debug, Clone)]
pub struct MdProps {
    pub uri: Option<String>,
}

pub enum Msg {
    MarkdownArrived(String),
    MarkdownFetchFailed,
}

impl Component for MarkdownWindow {
    type Message = Msg;
    type Properties = MdProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MarkdownWindow {
            fetch_service: FetchService::new(),
            fetch_task: None,
            markdown: None,
            props,
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::MarkdownArrived(md) => {
                log::info!("fetching markdown succeeded");
                self.markdown = Some(md)
            }
            Msg::MarkdownFetchFailed => log::error!("fetching markdown failed"),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        log::trace!("Change props: {:?}", props);
        self.props = props;
        self.try_fetch_markdown();
        true
    }

    fn view(&self) -> VNode {
        if let Some(md) = &self.markdown {
            html! {
                render_markdown(md)
            }
        } else {
            html! {}
        }
    }
}

impl MarkdownWindow {
    fn try_fetch_markdown(&mut self) {
        if let Some(uri) = &self.props.uri {
            log::info!("Getting new markdown");
            let request = Request::get(uri).body(Nothing).unwrap();
            let callback = self.link.callback(|response: Response<Text>| {
                log::info!("Got response");
                match response.body() {
                    Ok(text) => Msg::MarkdownArrived(text.clone()),
                    _ => Msg::MarkdownFetchFailed,
                }
            });
            self.fetch_task = self.fetch_service.fetch(request, callback).ok();
        }
    }
}
