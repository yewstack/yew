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

    /// The number of rounds to run.
    #[clap(long, default_value_t = 10)]
    rounds: usize,
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

impl Statistics {
    fn from_results<S>(name: S, round: usize, mut results: Vec<Duration>) -> Self
    where
        S: Into<String>,
    {
        let name = name.into();

        results.sort();

        let var: Variance = results.iter().cloned().map(dur_as_millis_f64).collect();

        Self {
            name,
            round: round.to_string(),
            min: format!("{:.3}", dur_as_millis_f64(results[0])),
            max: format!(
                "{:.3}",
                dur_as_millis_f64(*results.last().expect("array is empty?"))
            ),
            std_dev: format!("{:.3}", var.sample_variance().sqrt()),
            mean: format!("{:.3}", var.mean()),
        }
    }
}

fn create_progress(tests: usize, rounds: usize) -> ProgressBar {
    let bar = ProgressBar::new((tests * rounds) as u64);
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
                rounds
            ))
            .expect("failed to parse template")
            // .tick_chars("-\\|/")
            .progress_chars("=>-"),
    );

    bar
}

#[tokio::main]
async fn main() {
    let local_set = LocalSet::new();

    let args = Args::parse();

    // Tests in each round.
    static TESTS: usize = 4;

    let mut baseline_results = Vec::with_capacity(args.rounds);
    let mut hello_world_results = Vec::with_capacity(args.rounds);
    let mut function_router_results = Vec::with_capacity(args.rounds);
    let mut concurrent_tasks_results = Vec::with_capacity(args.rounds);

    let bar = (!args.no_term).then(|| create_progress(TESTS, args.rounds));

    local_set
        .run_until(async {
            for i in 0..=args.rounds {
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

    let output = [
        Statistics::from_results("Baseline", args.rounds, baseline_results),
        Statistics::from_results("Hello World", args.rounds, hello_world_results),
        Statistics::from_results("Function Router", args.rounds, function_router_results),
        Statistics::from_results("Concurrent Task", args.rounds, concurrent_tasks_results),
    ];

    println!("{}", output.as_ref().table().with(Style::rounded()));

    if let Some(ref p) = args.output_path {
        let mut f = File::create(p).expect("failed to write output.");
        serde_json::to_writer_pretty(&mut f, &output).expect("failed to write output.");

        println!();
        println!("Result has been written to: {}", p.display());
    }
}
