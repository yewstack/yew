#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

(cd yew \
  && cargo test --target wasm32-unknown-unknown --features wasm_test \
  && cargo test --target wasm32-unknown-unknown --features wasm_test \
    --features std_web,agent,services --no-default-features \
  && cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,toml \
  && cargo test --doc --features doc_test,wasm_test,yaml,msgpack,cbor,toml \
    --features std_web,agent,services --no-default-features)

(cd yew-functional \
  && cargo test --target wasm32-unknown-unknown)

(cd yew-macro \
  && cargo test --test macro_test \
  && cargo test --test derive_props_test \
  && cargo test --doc)

# TODO - Emscripten builds are broken on rustc > 1.39.0
if [ "$emscripten_supported" == "0" ]; then
  (cd yew \
    && cargo web test --target asmjs-unknown-emscripten \
      --features std_web,agent,services --no-default-features \
    && cargo web test --target wasm32-unknown-emscripten \
      --features std_web,agent,services --no-default-features)
fi
