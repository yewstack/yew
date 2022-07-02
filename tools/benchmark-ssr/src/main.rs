use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

use average::Variance;
use function_router::{ServerApp, ServerAppProps};
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use tabled::{Style, TableIteratorExt, Tabled};
use yew::platform::LocalRuntime;
use yew::prelude::*;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

static OUTPUT_JSON: Lazy<bool> = Lazy::new(|| {
    env::var("BENCH_OUTPUT_JSON")
        .map(|m| m == "1")
        .unwrap_or(false)
});

macro_rules! human_println {
    () => {
        if !*OUTPUT_JSON {
            println!();
        }
     };
    ($($arg:tt)*) => {
        if !*OUTPUT_JSON {
            println!($($arg)*);
        }
     };
}

fn dur_to_millis(dur: Duration) -> f64 {
    i32::try_from(dur.as_micros()).map(f64::from).unwrap() / 1000.0
}

fn bench_baseline() -> Duration {
    fn fib(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

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

#[derive(Debug, Tabled)]
struct Statistics {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Min (ms)")]
    min: String,
    #[tabled(rename = "Max (ms)")]
    max: String,
    #[tabled(rename = "Mean (ms)")]
    mean: String,
    #[tabled(rename = "Standard Deviation")]
    std_dev: String,
}

fn main() {
    let mut baseline_results = Vec::new();
    let mut hello_world_results = Vec::new();
    let mut function_router_results = Vec::new();

    let bar = ProgressBar::new(30);

    {
        let bar = bar.downgrade();
        std::thread::spawn(move || {
            while let Some(bar) = bar.upgrade() {
                bar.tick();
                std::thread::sleep(Duration::from_millis(100));
            }
        });
    }

    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} {prefix} [{elapsed_precise}] [{bar:40.cyan/blue}] round {msg}/10",
            )
            // .tick_chars("-\\|/")
            .progress_chars("=>-"),
    );

    for i in 0..=10 {
        bar.set_message(i.to_string());
        if i == 0 {
            bar.set_prefix("Warming up");
        } else {
            bar.set_prefix("Running   ");
        }

        let dur = bench_baseline();
        if i > 0 {
            baseline_results.push(dur);
            bar.inc(1);
        }

        let dur = bench_hello_world();
        if i > 0 {
            hello_world_results.push(dur);
            bar.inc(1);
        }

        let dur = bench_router_app();
        if i > 0 {
            function_router_results.push(dur);
            bar.inc(1);
        }
    }

    bar.finish_and_clear();
    drop(bar);

    baseline_results.sort();
    hello_world_results.sort();
    function_router_results.sort();

    let base_var: Variance = baseline_results
        .iter()
        .cloned()
        .map(dur_to_millis)
        .collect();

    let hw_var: Variance = hello_world_results
        .iter()
        .cloned()
        .map(dur_to_millis)
        .collect();

    let fr_var: Variance = function_router_results
        .iter()
        .cloned()
        .map(dur_to_millis)
        .collect();

    let table = [
        Statistics {
            name: "Baseline".into(),
            min: format!("{:.3}", dur_to_millis(baseline_results[0])),
            max: format!("{:.3}", dur_to_millis(baseline_results[9])),
            std_dev: format!("{:.3}", base_var.sample_variance().sqrt()),
            mean: format!("{:.3}", base_var.mean()),
        },
        Statistics {
            name: "Hello World".into(),
            min: format!("{:.3}", dur_to_millis(hello_world_results[0])),
            max: format!("{:.3}", dur_to_millis(hello_world_results[9])),
            std_dev: format!("{:.3}", hw_var.sample_variance().sqrt()),
            mean: format!("{:.3}", hw_var.mean()),
        },
        Statistics {
            name: "Function Router".into(),
            min: format!("{:.3}", dur_to_millis(function_router_results[0])),
            max: format!("{:.3}", dur_to_millis(function_router_results[9])),
            std_dev: format!("{:.3}", fr_var.sample_variance().sqrt()),
            mean: format!("{:.3}", fr_var.mean()),
        },
    ]
    .table()
    // .with(Header("Statistics"))
    .with(Style::rounded());

    human_println!("{}", table.to_string());
}
