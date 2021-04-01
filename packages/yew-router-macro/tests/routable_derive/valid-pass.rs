#[derive(Debug, PartialEq, yew_router::Routable)]
enum Routes {
    #[at("/")]
    One,
    #[at("/two/:id")]
    Two { id: u32 },
    #[at("/404")]
    NotFound,
}

fn main() {
    let _ = Routes::ONE;
    let _ = Routes::TWO;
    let _ = Routes::NOT_FOUND;
}
