#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

if [ "$emscripten_supported" == "0" ]; then
  # TODO - Emscripten builds are broken on rustc > 1.39.0
  cargo web test --features web_test --target asmjs-unknown-emscripten
  cargo web test --features web_test --target wasm32-unknown-emscripten
fi

cargo test --features wasm_test --target wasm32-unknown-unknown
cargo test --test macro_test
cargo test --test derive_props_test
cargo test --doc --all-features
(cd crates/macro && cargo test --doc)
