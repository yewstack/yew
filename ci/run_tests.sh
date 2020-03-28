#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

cargo test --target wasm32-unknown-unknown --features wasm_test,std_web
cargo test --target wasm32-unknown-unknown --features wasm_test,web_sys

if [ "$emscripten_supported" == "0" ]; then
  # TODO - Emscripten builds are broken on rustc > 1.39.0
  cargo web test --target asmjs-unknown-emscripten --features std_web
  cargo web test --target wasm32-unknown-emscripten --features std_web
fi

cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,std_web
cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,web_sys

(cd crates/macro \
  && cargo test --test macro_test \
  && cargo test --test derive_props_test \
  && cargo test --doc)

(cd crates/functional \
  && cargo test --features wasm_test --target wasm32-unknown-unknown)
