use yew::prelude::*;

#[function_component(InfoFooter)]
pub fn info_footer() -> Html {
    html! {
        <footer class="info">
            <p>{ "Double-click to edit a todo" }</p>
            <p>{ "Written by " }<a href="https://github.com/Yoroshikun/" target="_blank">{ "Drew Hutton <Yoroshi>" }</a></p>
            <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
        </footer>
    }
}
