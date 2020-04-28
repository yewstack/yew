#!/usr/bin/env bash

# src: https://gist.github.com/fbucek/f986da3cc3a9bbbd1573bdcb23fed2e1
set -e # error -> trap -> exit
function info() { echo -e "[\033[0;34m $@ \033[0m]"; } # blue: [ info message ]
function pass() { echo -e "[\033[0;32mPASS\033[0m] $@"; } # green: [PASS]
function fail() { FAIL="true"; echo -e "[\033[0;31mFAIL\033[0m] $@"; } # red: [FAIL]
trap 'LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ "$FAIL" == "true"  ]; then fail finished with error; else echo -e "[\033[0;32m Finished \033[0m]";fi' EXIT
SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir

cd $SRCDIR # ensure current dir is this dir

EXAMPLE=$1
cd $EXAMPLE


if [[ $EXAMPLE == *_wb ]]; then
    # Remove -wb from variable
    #EXAMPLE="${EXAMPLE%-wb}"

    info "Using wasm-bindgen"
    # When CARGO_TARGET_DIR is set
    TARGET_DIR=../target/wasm32-unknown-unknown/release
    if [ ! -z "$CARGO_TARGET_DIR" ]; then 
        TARGET_DIR=$CARGO_TARGET_DIR/wasm32-unknown-unknown/release
    fi

    info "Cargo build"
    cargo build --release --target wasm32-unknown-unknown 
    info "wasm-bindgen"
    wasm-bindgen --target web --no-typescript --out-dir $SRCDIR/static/ --out-name wasm $TARGET_DIR/$EXAMPLE.wasm
    # @see https://rustwasm.github.io/book/game-of-life/code-size.html
    mv $SRCDIR/static/wasm_bg.wasm $SRCDIR/static/wasm_bg_orig.wasm
    # Optimalization 
    # -Os -> size
    wasm-opt $SRCDIR/static/wasm_bg_orig.wasm -Os -o $SRCDIR/static/wasm_bg.wasm
else 
    info "Using wasm-pack"
    wasm-pack build --debug --target web --out-name wasm --out-dir ../static
fi
