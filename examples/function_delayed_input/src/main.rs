use gloo_timers::callback::Timeout;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::*;

#[component]
fn App() -> Html {
    let search = use_state(String::new);
    let response_data = use_state(String::new);

    use_effect_with(search.clone(), {
        let search = search.clone();
        let response_data = response_data.clone();
        move |_| {
            // here you would typically do a REST call to send the search input to backend
            // for simplicity sake here we just set back the original input
            response_data.set((*search).clone())
        }
    });

    let oninput = Callback::from({
        let timeout_ref = use_mut_ref(|| None);
        let search = search.clone();
        move |e: InputEvent| {
            if let Some(target) = e.target() {
                let input = target.dyn_into::<HtmlInputElement>().ok();
                if let Some(input) = input {
                    let value = input.value();
                    if !value.is_empty() {
                        let search = search.clone();
                        let timeout = Timeout::new(1_000, move || {
                            search.set(value);
                        });
                        timeout_ref.borrow_mut().replace(timeout);
                    }
                }
            }
        }
    });

    html! {
      <div class="container p-2">
          <div class="row">
            <div class="p-2">
              <form class="input-group bg-dark border border-white rounded">
                <input id="search" autocomplete="off" type="search" class="form-control" placeholder="Type something here..." aria-label="Search" {oninput}/>
              </form>
            </div>
            <div class="p-2 border border-black rounded">
              <p class="mb-0">{"The input value will appear below after a timeout:"}</p>
              <p>{&*response_data}</p>
            </div>
          </div>
      </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
