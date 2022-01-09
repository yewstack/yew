use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use std::io;
use std::io::{Read, Write};

#[derive(Serialize)]
struct GhActionBenchmark {
    name: String,
    unit: String,
    value: String,
}

fn main() -> Result<()> {
    let mut buffer = "".to_string();
    io::stdin().read_to_string(&mut buffer)?;

    let input_json: Vec<Value> = serde_json::from_str(buffer.as_str())?;

    let transformed_benchmarks: Vec<GhActionBenchmark> = input_json
        .into_iter()
        .map(|v| GhActionBenchmark {
            name: format!("{} {}", v["framework"], v["benchmark"]).replace("\"", ""),
            unit: String::default(),
            value: v["median"].to_string(),
        })
        .collect();

    let output = serde_json::to_string(&transformed_benchmarks)?;

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}
