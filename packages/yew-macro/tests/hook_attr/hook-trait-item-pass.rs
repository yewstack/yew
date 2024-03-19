use std::marker::PhantomData;

pub struct QueryState<T> {
    p: PhantomData<T>
}

pub trait MyTrait {
    type Associated;
}


#[yew::hook]
pub fn use_query_state<Props>(
    selector: impl yew::html::IntoPropValue<bool>,
) -> QueryState<Props::Associated>
where
    Props: MyTrait,
{
    QueryState::<Props::Associated> { p: PhantomData::<Props::Associated> }
}

fn main() {}