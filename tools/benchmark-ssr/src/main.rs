use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use average::Variance;
use clap::Parser;
use function_router::{ServerApp, ServerAppProps};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use tabled::{Style, TableIteratorExt, Tabled};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::task::{spawn_local, LocalSet};
use yew::platform::time::sleep;
use yew::prelude::*;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Parser)]
struct Args {
    /// Disable terminal support.
    #[clap(long)]
    no_term: bool,
    /// Write the report to an output path in json format.
    #[clap(long)]
    output_path: Option<PathBuf>,
}

fn dur_as_millis_f64(dur: Duration) -> f64 {
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

async fn bench_hello_world() -> Duration {
    static TOTAL: usize = 1_000_000;

    #[function_component]
    fn App() -> Html {
        html! {<div>{"Hello, World!"}</div>}
    }

    let start_time = Instant::now();

    for _ in 0..TOTAL {
        yew::LocalServerRenderer::<App>::new().render().await;
    }

    start_time.elapsed()
}

async fn bench_router_app() -> Duration {
    static TOTAL: usize = 100_000;

    let start_time = Instant::now();

    for _ in 0..TOTAL {
        yew::LocalServerRenderer::<ServerApp>::with_props(ServerAppProps {
            url: "/".into(),
            queries: HashMap::new(),
        })
        .render()
        .await;
    }

    start_time.elapsed()
}

async fn bench_concurrent_task() -> Duration {
    static TOTAL: usize = 100;

    let start_time = Instant::now();

    #[function_component]
    fn Comp() -> HtmlResult {
        let _state = use_prepared_state!(
            async move |_| -> () {
                sleep(Duration::from_secs(1)).await;
            },
            ()
        )?;

        Ok(Html::default())
    }

    #[function_component]
    fn Parent() -> Html {
        html! {
            <>
                <Comp />
                <Comp />
                <Comp />
                <Comp />
            </>
        }
    }

    #[function_component]
    fn App() -> Html {
        html! {
            <Suspense fallback={Html::default()}>
                <Parent />
                <Comp />
                <Comp />
                <Comp />
                <Comp />
            </Suspense>
        }
    }

    let mut tasks = Vec::new();

    for _ in 0..TOTAL {
        tasks.push(spawn_local(async {
            yew::LocalServerRenderer::<App>::new().render().await;
        }));
    }

    for task in tasks {
        task.await.expect("failed to finish task");
    }

    start_time.elapsed()
}

#[derive(Debug, Tabled, Serialize, Deserialize)]
struct Statistics {
    #[tabled(rename = "Benchmark")]
    name: String,
    #[tabled(rename = "Round")]
    round: String,
    #[tabled(rename = "Min (ms)")]
    min: String,
    #[tabled(rename = "Max (ms)")]
    max: String,
    #[tabled(rename = "Mean (ms)")]
    mean: String,
    #[tabled(rename = "Standard Deviation")]
    std_dev: String,
}

static ROUND: u16 = 10;

fn create_bar() -> ProgressBar {
    // There are 3 items per round.
    let bar = ProgressBar::new(u64::from(ROUND * 4));
    // Progress Bar needs to be updated in a different thread.
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
            .template(&format!(
                "{{spinner:.green}} {{prefix}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] round \
                 {{msg}}/{}",
                ROUND
            ))
            // .tick_chars("-\\|/")
            .progress_chars("=>-"),
    );

    bar
}

#[tokio::main]
async fn main() {
    let local_set = LocalSet::new();

    let args = Args::parse();

    let mut baseline_results = Vec::new();
    let mut hello_world_results = Vec::new();
    let mut function_router_results = Vec::new();
    let mut concurrent_tasks_results = Vec::new();

    let bar = (!args.no_term).then(create_bar);

    local_set
        .run_until(async {
            for i in 0..=ROUND {
                if let Some(ref bar) = bar {
                    bar.set_message(i.to_string());
                    if i == 0 {
                        bar.set_prefix("Warming up");
                    } else {
                        bar.set_prefix("Running   ");
                    }
                }

                let dur = bench_baseline();
                if i > 0 {
                    baseline_results.push(dur);
                    if let Some(ref bar) = bar {
                        bar.inc(1);
                    }
                }

                let dur = bench_hello_world().await;
                if i > 0 {
                    hello_world_results.push(dur);
                    if let Some(ref bar) = bar {
                        bar.inc(1);
                    }
                }

                let dur = bench_router_app().await;
                if i > 0 {
                    function_router_results.push(dur);
                    if let Some(ref bar) = bar {
                        bar.inc(1);
                    }
                }

                let dur = bench_concurrent_task().await;
                if i > 0 {
                    concurrent_tasks_results.push(dur);
                    if let Some(ref bar) = bar {
                        bar.inc(1);
                    }
                }
            }
        })
        .await;

    if let Some(ref bar) = bar {
        bar.finish_and_clear();
    }
    drop(bar);

    baseline_results.sort();
    hello_world_results.sort();
    function_router_results.sort();
    concurrent_tasks_results.sort();

    let base_var: Variance = baseline_results
        .iter()
        .cloned()
        .map(dur_as_millis_f64)
        .collect();

    let hw_var: Variance = hello_world_results
        .iter()
        .cloned()
        .map(dur_as_millis_f64)
        .collect();

    let fr_var: Variance = function_router_results
        .iter()
        .cloned()
        .map(dur_as_millis_f64)
        .collect();

    let ct_var: Variance = concurrent_tasks_results
        .iter()
        .cloned()
        .map(dur_as_millis_f64)
        .collect();

    let output = [
        Statistics {
            name: "Baseline".into(),
            round: ROUND.to_string(),
            min: format!("{:.3}", dur_as_millis_f64(baseline_results[0])),
            max: format!("{:.3}", dur_as_millis_f64(baseline_results[9])),
            std_dev: format!("{:.3}", base_var.sample_variance().sqrt()),
            mean: format!("{:.3}", base_var.mean()),
        },
        Statistics {
            name: "Hello World".into(),
            round: ROUND.to_string(),
            min: format!("{:.3}", dur_as_millis_f64(hello_world_results[0])),
            max: format!("{:.3}", dur_as_millis_f64(hello_world_results[9])),
            std_dev: format!("{:.3}", hw_var.sample_variance().sqrt()),
            mean: format!("{:.3}", hw_var.mean()),
        },
        Statistics {
            name: "Function Router".into(),
            round: ROUND.to_string(),
            min: format!("{:.3}", dur_as_millis_f64(function_router_results[0])),
            max: format!("{:.3}", dur_as_millis_f64(function_router_results[9])),
            std_dev: format!("{:.3}", fr_var.sample_variance().sqrt()),
            mean: format!("{:.3}", fr_var.mean()),
        },
        Statistics {
            name: "Concurrent Task".into(),
            round: ROUND.to_string(),
            min: format!("{:.3}", dur_as_millis_f64(concurrent_tasks_results[0])),
            max: format!("{:.3}", dur_as_millis_f64(concurrent_tasks_results[9])),
            std_dev: format!("{:.3}", ct_var.sample_variance().sqrt()),
            mean: format!("{:.3}", ct_var.mean()),
        },
    ];

    println!("{}", output.as_ref().table().with(Style::rounded()));

    if let Some(ref p) = args.output_path {
        let mut f = File::create(p).expect("failed to write output.");
        serde_json::to_writer_pretty(&mut f, &output).expect("failed to write output.");

        println!();
        println!("Result has been written to: {}", p.display());
    }
}
