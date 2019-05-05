use yew::{Renderable, Component, ComponentLink, ToRenderableOption, ShouldRender, Html, html};

pub struct OptionalRendering {
    menu_shown: bool,
    content_shown: bool,
}

pub enum ComponentMessage {
    ToggleMenu,
    ToggleContent,
}

impl Component for OptionalRendering {
    type Properties = ();
    type Message = ComponentMessage; 

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        OptionalRendering {
            menu_shown: false,
            content_shown: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ComponentMessage::ToggleMenu => self.menu_shown = !self.menu_shown,
            ComponentMessage::ToggleContent => self.content_shown = !self.content_shown,
        }

        true
    }
}

impl OptionalRendering {
    fn menu(&self) -> Option<Html<Self>> {
        if !self.menu_shown {
            return None;
        }

        Some(html! {
            <nav>
                <ul>
                    <li>{ "Main Page" }</li>
                    <li>{ "Subpage" }</li>
                    <li>{ "Impressum" }</li>
                </ul>
            </nav>
        })
    }

    fn content(&self) -> Option<Html<Self>> {
        if !self.content_shown {
            return None;
        }

        Some(html! {
            <>
              <style>
                  { "table, td { border: 1px solid Black; } " }
              </style>
              <table>
                  { for (0..10).map(|_| html! {
                      <tr>
                          { for (0..10).map(|i| html! { <td>{ i }</td> }) }
                      </tr>
                  })}
              </table>
            </>
        })
    }
}

impl Renderable<OptionalRendering> for OptionalRendering {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <button onclick=|_| ComponentMessage::ToggleMenu,>{ "Toggle Menu" }</button>
                    <button onclick=|_| ComponentMessage::ToggleContent,>{ "Toggle Content" }</button>
                </div>
                { self.menu().renderable() }
                { self.content().renderable() }
            </div>
        }
    }
}

