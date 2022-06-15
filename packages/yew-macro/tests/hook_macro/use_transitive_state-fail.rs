use yew::prelude::*;
use yew_macro::{use_transitive_state_with_closure, use_transitive_state_without_closure};

#[function_component]
fn Comp() -> HtmlResult {
    use_transitive_state_with_closure!(123)?;

    use_transitive_state_with_closure!(|_| { todo!() }, 123)?;

    use_transitive_state_with_closure!(|_| -> u32 { todo!() })?;

    Ok(Html::default())
}

#[function_component]
fn Comp2() -> HtmlResult {
    use_transitive_state_without_closure!(123)?;

    use_transitive_state_without_closure!(|_| { todo!() }, 123)?;

    use_transitive_state_without_closure!(|_| -> u32 { todo!() })?;

    Ok(Html::default())
}

fn main() {}
