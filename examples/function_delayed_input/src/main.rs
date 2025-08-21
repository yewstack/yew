use std::time::Duration;

use gloo_timers::callback::Timeout;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::*;

#[component]
fn App() -> Html {
    #[derive(PartialEq, Default, Clone)]
    enum Search {
        #[default]
        Idle,
        Fetching(AttrValue),
        Fetched(AttrValue),
    }

    let search = use_state(Search::default);

    use_effect_with(search.clone(), {
        move |search| {
            // here you would typically do a REST call to send the search input to backend
            // for simplicity sake here we just set back the original input
            if let Search::Fetching(query) = &**search {
                yew::platform::spawn_local({
                    let query = query.clone();
                    let search = search.setter();
                    async move {
                        // Simulate a network delay
                        gloo_timers::future::sleep(Duration::from_millis(500)).await;
                        search.set(Search::Fetched(
                            format!("Placeholder response for: {}", query).into(),
                        ));
                    }
                });
            }
        }
    });

    let oninput = {
        let timeout_ref = use_mut_ref(|| None);
        use_callback((), {
            let search = search.clone();
            move |e: InputEvent, _| {
                if let Some(target) = e.target() {
                    let input = target.dyn_into::<HtmlInputElement>().ok();
                    if let Some(input) = input {
                        let value = input.value();
                        if !value.is_empty() {
                            let search = search.setter();
                            let timeout = Timeout::new(1_000, move || {
                                search.set(Search::Fetching(value.into()));
                            });
                            (*timeout_ref.borrow_mut()) = Some(timeout);
                        }
                    }
                }
            }
        })
    };

    html! {
      <div class="container p-2">
          <div class="row">
            <div class="p-2">
              <form class="input-group bg-dark border border-white rounded">
                <input id="search" autocomplete="off" type="search" class="form-control" placeholder="Type something here..." aria-label="Search" {oninput}/>
              </form>
            </div>
            <div class="p-2 border border-black rounded">
                <p>{
                    match &*search {
                        Search::Idle => "Type something to search...".into(),
                        Search::Fetching(query) => format!("Searching for: {}", query).into(),
                        Search::Fetched(response) => response.clone(),
                    }
                }</p>
            </div>
          </div>
      </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
