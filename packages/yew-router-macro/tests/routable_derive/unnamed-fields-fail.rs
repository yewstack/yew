#[derive(Clone, yew_router::Routable)]
enum Routes {
    #[at("/one/:two")]
    One(u32),
}

fn main() {}
