use std::collections::HashMap;
use std::time::{Duration, Instant};

use function_router::{ServerApp, ServerAppProps};
use yew::platform::LocalRuntime;
use yew::prelude::*;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn fib(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn baseline() -> Duration {
    let start_time = Instant::now();
    fib(40);
    start_time.elapsed()
}

fn bench_hello_world() -> Duration {
    static TOTAL: usize = 1_000_000;

    #[function_component]
    fn App() -> Html {
        html! {<div>{"Hello, World!"}</div>}
    }

    let rt = LocalRuntime::new().expect("failed to create runtime.");

    let start_time = Instant::now();

    rt.block_on(async move {
        for _ in 0..TOTAL {
            yew::LocalServerRenderer::<App>::new().render().await;
        }
    });

    start_time.elapsed()
}

fn bench_router_app() -> Duration {
    static TOTAL: usize = 100_000;

    let start_time = Instant::now();

    let rt = LocalRuntime::new().expect("failed to create runtime.");

    rt.block_on(async move {
        for _ in 0..TOTAL {
            yew::LocalServerRenderer::<ServerApp>::with_props(ServerAppProps {
                url: "/".into(),
                queries: HashMap::new(),
            })
            .render()
            .await;
        }
    });

    start_time.elapsed()
}

fn print_result(name: &str, dur: Duration) {
    let dur_millis = i32::try_from(dur.as_micros()).map(f64::from).unwrap() / 1000.0;

    println!("{}: {:.3}ms", name, dur_millis);
}

fn main() {
    let mut baselines = Vec::with_capacity(10);
    let mut hello_world_apps = Vec::with_capacity(10);
    let mut function_router_apps = Vec::with_capacity(10);

    for _ in 0..10 {
        let baseline = baseline();
        baselines.push(baseline);
        print_result("Baseline", baseline);
        let bench_hello_world = bench_hello_world();
        hello_world_apps.push(bench_hello_world);
        print_result("Hello World (1,000,000 runs)", bench_hello_world);
        let bench_router_app = bench_router_app();
        function_router_apps.push(bench_router_app);
        print_result("Function Router (100,000 runs)", bench_router_app);
    }

    print_result(
        "Baseline",
        baselines.into_iter().fold(Duration::ZERO, |l, r| l + r),
    );
    print_result(
        "Hello World (10,000,000 runs)",
        hello_world_apps
            .into_iter()
            .fold(Duration::ZERO, |l, r| l + r),
    );
    print_result(
        "Function Router (1,000,000 runs)",
        function_router_apps
            .into_iter()
            .fold(Duration::ZERO, |l, r| l + r),
    );
}
