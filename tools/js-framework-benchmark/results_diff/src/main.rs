use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
struct Result {
    framework: String,
    benchmark: String,
    values: Vec<f64>,
}

#[derive(Default)]
struct BenchmarkResults {
    bindgen: f64,
    baseline: f64,
    update: f64,
}

fn main() {
    let result_str = include_str!("../../webdriver-ts/results.json");
    let results: Vec<Result> = serde_json::from_str(result_str).expect("failed to deserialize");
    let mut benchmark_results: BTreeMap<String, BenchmarkResults> = BTreeMap::new();
    for result in results.into_iter() {
        let value_sum: f64 = result.values.iter().sum();
        let value_count = result.values.len();
        let avg_val = if value_count > 0 {
            value_sum / value_count as f64
        } else {
            0f64
        };

        let mut entry = benchmark_results.entry(result.benchmark).or_default();
        if result.framework.starts_with("wasm-bindgen") {
            entry.bindgen = avg_val
        } else if result.framework.starts_with("yew-baseline") {
            entry.baseline = avg_val;
        } else if result.framework.starts_with("yew") {
            entry.update = avg_val;
        }
    }

    let max_benchmark_name_len = benchmark_results.keys().map(|key| key.len()).max().unwrap();
    let full_length = max_benchmark_name_len + 35 + 4 * 3 + 2; // 35: sum of columns defined below, 3: padding, 2: sign

    println!("### Benchmark Report");
    println!("- `wasm-bindgen`: the performance goal");
    println!("- `baseline`: performance of `yew-baseline` (typically latest master)");
    println!("- `update`: performance of `yew` (typically recent changes)");
    println!("- `diff`: measures the improvement of `update` over the `baseline`");
    println!("```diff");
    println!("@@ {:^1$} @@", "Performance Diff", full_length - 6);
    println!();
    println!("##{:>1$} | wasm-bindgen | baseline |  update |  diff ##", "", max_benchmark_name_len);
    println!("{}", "#".repeat(full_length));
    for (benchmark, results) in benchmark_results {
        let diff = (results.update - results.baseline) / results.baseline; // TODO zero check
        let sign = if diff < 0f64 { "+"} else { "-"};
        print!("{} {:<2$} | ", sign, benchmark, max_benchmark_name_len);
        print!("{:>1$} | ", format!("{:.2}", results.bindgen), 12); // 12: wasm-bindgen
        print!("{:>1$} | ", format!("{:.2}", results.baseline), 8); // 8: baseline
        print!("{:>1$} | ", format!("{:.2}", results.update), 7); // 7: f64 spacing
        println!("{:>1$}", format!("{:+.2}%", 100f64 * diff), 8); // 8: pct spacing
    }
    println!("```");
}
