#!/bin/bash

# Originally this ci script borrowed from https://github.com/koute/stdweb
# because both use `cargo-web` tool to check the compilation.

set -euo pipefail
IFS=$'\n\t'

set +e
echo "$(rustc --version)" | grep -q "nightly"
if [ "$?" = "0" ]; then
    export IS_NIGHTLY=1
else
    export IS_NIGHTLY=0
fi
set -e

echo "Is Rust from nightly: $IS_NIGHTLY"

echo "Testing for asmjs-unknown-emscripten..."
cargo web test --features web_test --target=asmjs-unknown-emscripten

echo "Testing for wasm32-unknown-emscripten..."
cargo web test --features web_test --target=wasm32-unknown-emscripten

if [ "$IS_NIGHTLY" = "1" ]; then
    echo "Testing for wasm32-unknown-unknown..."
    cargo web test --nodejs --target=wasm32-unknown-unknown
fi

check_example() {
    echo "Checking example [$2]"
    pushd $2 > /dev/null
    cargo web build --target=$1
    popd > /dev/null

    # TODO Can't build some demos with release, need fix
    # cargo web build --release $CARGO_WEB_ARGS
}

check_all_examples() {
    echo "Checking examples on $1..."
    for EXAMPLE in $(pwd)/examples/*; do
        if [ "$1" == "wasm32-unknown-unknown" ]; then
            # The counter example doesn't yet build here.
            case $(basename $EXAMPLE) in
                "counter")
                continue
                ;;
                *)
                ;;
            esac
        fi

        if [ -d "$EXAMPLE" ]; then
            check_example $1 $EXAMPLE
        fi
    done
}


check_all_examples asmjs-unknown-emscripten
check_all_examples wasm32-unknown-emscripten

if [ "$IS_NIGHTLY" = "1" ]; then
    check_all_examples wasm32-unknown-unknown
fi
