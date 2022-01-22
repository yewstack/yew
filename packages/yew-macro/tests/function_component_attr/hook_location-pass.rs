use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Ctx;

#[function_component]
fn Comp() -> Html {
    use_context::<Ctx>().unwrap();

    if let Some(_m) = use_context::<Ctx>() {
        todo!()
    }

    let _ctx = { use_context::<Ctx>() };

    match use_context::<Ctx>() {
        Some(_) => {
            todo!()
        }
        None => {
            todo!()
        }
    }
}

fn main() {}
