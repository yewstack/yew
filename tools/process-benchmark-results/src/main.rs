use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
struct GhActionBenchmark {
    name: String,
    unit: String,
    value: Value,
}

// from https://github.com/krausest/js-framework-benchmark/blob/master/webdriver-ts/src/writeResults.ts#L67 function createResultFile
#[derive(Deserialize)]
struct ResultData {
    median: Value,
    // some keys missing
}

#[derive(Deserialize)]
struct JsKrauseBenchmarkResult<'r> {
    framework: &'r str,
    benchmark: &'r str,
    r#type: &'r str,
    values: HashMap<&'r str, ResultData>,
}

fn transform_results(mut result: JsKrauseBenchmarkResult<'_>) -> GhActionBenchmark {
    let key = if result.r#type == "cpu" {
        "total"
    } else {
        "DEFAULT"
    };
    let values = result.values.remove(key).unwrap_or_else(|| {
        panic!(
            "Expect benchmark data to be present for type {}. Found keys: {:?}, expected {key:?}",
            result.r#type,
            result.values.keys().cloned().collect::<Vec<_>>(),
        )
    });
    assert!(
        values.median.is_number(),
        "expected a numerical benchmark value"
    );
    GhActionBenchmark {
        name: format!("{} {}", result.framework, result.benchmark).replace('"', ""),
        unit: String::default(),
        value: values.median,
    }
}

fn main() -> Result<()> {
    let mut buffer = "".to_string();
    io::stdin().read_to_string(&mut buffer)?;

    let input_json: Vec<_> = serde_json::from_str(buffer.as_str())?;

    let transformed_benchmarks: Vec<GhActionBenchmark> =
        input_json.into_iter().map(transform_results).collect();

    let output = serde_json::to_string(&transformed_benchmarks)?;

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}
