#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "stable"
is_stable=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

if [ "$is_stable" == "0" ]; then
  # TODO - Emscripten builds are broken on beta/nightly
  cargo web test --features web_test --target asmjs-unknown-emscripten
  cargo web test --features web_test --target wasm32-unknown-emscripten
fi

cargo test --features wasm_test --target wasm32-unknown-unknown
cargo test --test macro_test
cargo test --test derive_props_test
cargo doc_test --all-features
(cd crates/macro && cargo doc_test)
