#!/usr/bin/env bash
set -euo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

# SETUP

test_flags=("--headless" "--firefox")
test_features="wasm_bench"

echo "running benchmarks with flags: ${test_flags[*]} and features: ${test_features}"

# TESTS

set -x

(cd yew &&
  wasm-pack test "${test_flags[@]}" --release \
    -- --features "${test_features}" bench
)
