use yew::prelude::*;

#[hook]
fn use_reducer_default_action<T>() -> T::Action
where
    T: Reducible + 'static,
    T::Action: Default + 'static,
{
    T::Action::default()
}

fn main() {}
