#[derive(Debug, Clone, PartialEq, yew_router::Routable)]
enum Routes {
    #[at("/")]
    Home {
        #[bind(query)]
        #[bind(query)]
        id: u32
    },
}

fn main() {}
