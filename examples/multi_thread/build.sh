#!/usr/bin/env bash

# src: https://gist.github.com/fbucek/f986da3cc3a9bbbd1573bdcb23fed2e1
set -e # error -> trap -> exit
function info() { echo -e "[\033[0;34m $@ \033[0m]"; } # blue: [ info message ]
function fail() { FAIL="true"; echo -e "[\033[0;31mFAIL\033[0m] $@"; } # red: [FAIL]
trap 'LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ "$FAIL" == "true"  ]; then fail finished with error; else echo -e "[\033[0;32m Finished $@ \033[0m]";fi' EXIT
SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir

cd $SRCDIR # ensure current dir is this dir

TARGET_DIR=../../target/wasm32-unknown-unknown/release
if [ ! -z "$CARGO_TARGET_DIR" ]; then 
    TARGET_DIR=$CARGO_TARGET_DIR/wasm32-unknown-unknown/release
fi
info "Building multi_thread using wasm-bindgen"

info "Building app"
cargo build --release --target wasm32-unknown-unknown --bin multi_thread_app
wasm-bindgen --target web --no-typescript --out-dir static/ --out-name app $TARGET_DIR/multi_thread_app.wasm

info "Building worker"
cargo build --release --target wasm32-unknown-unknown --bin multi_thread_worker
wasm-bindgen --target web --no-typescript --out-dir static/ --out-name worker $TARGET_DIR/multi_thread_worker.wasm
info "Running server"
python3 -m http.server --directory static 
