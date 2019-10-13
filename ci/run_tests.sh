#!/usr/bin/env bash
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

cargo web test --features web_test --target asmjs-unknown-emscripten
cargo web test --features web_test --target wasm32-unknown-emscripten
cargo test --features wasm_test --target wasm32-unknown-unknown
cargo test --test macro_test
cargo test --test derive_props_test
cargo doc_test
(cd crates/macro && cargo doc_test)
