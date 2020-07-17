#!/usr/bin/env bash
set -euo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

# SETUP

test_flags=("--headless" "--firefox")
test_features="wasm_test"

if [[ -z "${HTTPBIN_URL-}" ]]; then
  echo "INFO: HTTPBIN_URL isn't set, won't run fetch service tests"
  echo "      please see the CONTRIBUTING.md files for instructions"
else
  echo "INFO: using '$HTTPBIN_URL' for fetch service tests"
  test_features+=",httpbin_test"
fi

echo "running tests with flags: ${test_flags[*]} and features: ${test_features}"

# TESTS

set -x

(cd yew &&
  wasm-pack test "${test_flags[@]}" -- --features "${test_features}" &&
  cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,toml &&
  cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,toml \
    --features std_web,agent,services --no-default-features)

(cd yew-functional && wasm-pack test "${test_flags[@]}")

(cd yew-macro &&
  cargo test --test macro_test &&
  cargo test --test derive_props_test &&
  cargo test --doc)

(cd yew-router && cargo test)
(cd yew-router-macro && cargo test)
(cd yew-router-route-parser && cargo test)

(cd yew-stdweb && wasm-pack test "${test_flags[@]}" -- --features "${test_features}")

(cd yewtil && cargo test)

(cd yew-components && cargo test)
