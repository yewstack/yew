#!/usr/bin/env bash

# The example to build.
EXAMPLE=${1%\/}
shift

# Optimization level. Can be either "--debug" or "--release". Defaults to debug.
PROFILE="--debug"

# Whether to open a browser window after building
START_BROWSER=1

while (("$#")); do
    case "$1" in
    --release)
        PROFILE="--release"
        shift
        ;;
    --debug)
        PROFILE="--debug"
        shift
        ;;
    --build-only)
        START_BROWSER=0
        shift
        ;;
    -*) # unsupported flags
        echo "Error: Unsupported flag $1" >&2
        exit 1
        ;;
    esac
done

# src: https://gist.github.com/fbucek/f986da3cc3a9bbbd1573bdcb23fed2e1
set -e # error -> trap -> exit

info() {
    # blue: [ info message ]
    echo -e "[\033[0;34m $* \033[0m]"
}
fail() {
    FAIL="true"
    # red: [FAIL]
    echo -e "[\033[0;31mFAIL\033[0m] $*"
}

on_exit() {
    LASTRES=$?
    LAST=$BASH_COMMAND
    if [[ LASTRES -ne 0 ]]; then
        fail "Command: \"$LAST\" exited with exit code: $LASTRES"
    elif [ "$FAIL" == "true" ]; then
        fail finished with error
    elif [[ $START_BROWSER != 1 ]]; then
        echo -e "[\033[0;32m Finished! Run $EXAMPLE by serving the generated files in examples/static/ \033[0m]"
    fi
}

trap on_exit EXIT

SRCDIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)" # this source dir

cd "$SRCDIR/$EXAMPLE" # "$SRCDIR" ensures that this script can be run from anywhere.

# When using $CARGO_TARGET_DIR -> binary is located in different folder
# Necessary to locate build files for wasm-bindgen
TARGET_DIR=$SRCDIR/../target/wasm32-unknown-unknown
if [ -n "$CARGO_TARGET_DIR" ]; then
    TARGET_DIR=$CARGO_TARGET_DIR/wasm32-unknown-unknown
fi
if [[ "$PROFILE" = "--release" ]]; then
    TARGET_DIR=$TARGET_DIR/release
else
    TARGET_DIR=$TARGET_DIR/debug
fi

# Build the correct cargo build command depending on the optimization level.
cargo_build() {
    if [[ "$PROFILE" = "--release" ]]; then
        cargo build --release --target wasm32-unknown-unknown "$@"
    else
        cargo build --target wasm32-unknown-unknown "$@"
    fi
}

# wasm-pack build
if [[ $EXAMPLE == *_wp ]]; then
    info "Building: $EXAMPLE using wasm-pack"
    wasm-pack build "$PROFILE" --target web --out-name wasm --out-dir "$SRCDIR/$EXAMPLE/static/"

# multi_thread build -> two binary/wasm files
elif [[ $EXAMPLE == multi_thread ]]; then
    info "Building: $EXAMPLE app using wasm-bindgen"
    cargo_build --bin multi_thread_app
    wasm-bindgen --target web --no-typescript --out-dir "$SRCDIR/$EXAMPLE/static/" --out-name wasm "$TARGET_DIR/multi_thread_app.wasm"

    info "Building: $EXAMPLE worker using wasm-bindgen"
    cargo_build --bin multi_thread_worker
    wasm-bindgen --target no-modules --no-typescript --out-dir "$SRCDIR/$EXAMPLE/static/" --out-name worker "$TARGET_DIR/multi_thread_worker.wasm"

else # Default wasm-bindgen build
    info "Building: $EXAMPLE using wasm-bindgen"
    cargo_build
    wasm-bindgen --target web --no-typescript --out-dir "$SRCDIR/$EXAMPLE/static/" --out-name wasm "$TARGET_DIR/$EXAMPLE.wasm"
fi

cd static
if [[ $START_BROWSER == 1 ]]; then
    if ! [ -x "$(command -v python3)" ]; then
        echo "WARNING: python3 not found! Please manually start a web server for the $SRCDIR/$EXAMPLE/static directory."
        echo "         Use '--build-only' to suppress this message."
        exit 1
    fi
    python3 ../../start_example_server.py $FLAGS
fi
