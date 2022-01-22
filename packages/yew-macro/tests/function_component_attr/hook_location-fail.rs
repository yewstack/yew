use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Ctx;

#[function_component]
fn Comp() -> Html {
    if let Some(_m) = use_context::<Ctx>() {
        use_context::<Ctx>().unwrap();
        todo!()
    }

    let _ = || {
        use_context::<Ctx>().unwrap();
        todo!()
    };

    for _ in 0..10 {
        use_context::<Ctx>().unwrap();
    }

    while let Some(_m) = use_context::<Ctx>() {
        use_context::<Ctx>().unwrap();
    }

    match use_context::<Ctx>() {
        Some(_) => use_context::<Ctx>(),
        None => {
            todo!()
        }
    }

    loop {
        use_context::<Ctx>().unwrap();
        todo!()
    }
}

fn main() {}
