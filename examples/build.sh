#!/usr/bin/env bash

# src: https://gist.github.com/fbucek/f986da3cc3a9bbbd1573bdcb23fed2e1
set -e # error -> trap -> exit
function info() { echo -e "[\033[0;34m $@ \033[0m]"; } # blue: [ info message ]
function fail() { FAIL="true"; echo -e "[\033[0;31mFAIL\033[0m] $@"; } # red: [FAIL]
trap 'LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ "$FAIL" == "true"  ]; then fail finished with error; else echo -e "[\033[0;32m Finished! Run $@ by serving the generated files in examples/static/ \033[0m]";fi' EXIT
SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir

cd $SRCDIR # ensure this script can be run from anywhere

# When using $CARGO_TARGET_DIR -> binary is located in different folder
# Necessary to locate build files for wasm-bindgen
TARGET_DIR=$SRCDIR/../target/wasm32-unknown-unknown/debug
if [ ! -z "$CARGO_TARGET_DIR" ]; then 
    TARGET_DIR=$CARGO_TARGET_DIR/wasm32-unknown-unknown/debug
fi

EXAMPLE=$1
cd $EXAMPLE

# wasm-pack build
if [[ $EXAMPLE == *_wp ]]; then 
    info "Building: $EXAMPLE using wasm-pack"
    # wasm-pack overwrites .gitignore -> save -> restore
    cp $SRCDIR/static/.gitignore $SRCDIR/static/.gitignore.copy
    wasm-pack build --debug --target web --out-name wasm --out-dir $SRCDIR/static/
    rm $SRCDIR/static/.gitignore; mv $SRCDIR/static/.gitignore.copy $SRCDIR/static/.gitignore # restore .gitignore

# multi_thread build -> two binary/wasm files
elif [[ $EXAMPLE == multi_thread ]]; then
    info "Building: $EXAMPLE app using wasm-bindgen"
    cargo build --target wasm32-unknown-unknown --bin multi_thread_app
    wasm-bindgen --target web --no-typescript --out-dir $SRCDIR/static/ --out-name wasm $TARGET_DIR/multi_thread_app.wasm

    info "Building: $EXAMPLE worker using wasm-bindgen"
    cargo build --target wasm32-unknown-unknown --bin multi_thread_worker
    wasm-bindgen --target no-modules --no-typescript --out-dir $SRCDIR/static/ --out-name worker $TARGET_DIR/multi_thread_worker.wasm

else # Default wasm-bindgen build
    info "Building: $EXAMPLE using wasm-bindgen"
    cargo build --target wasm32-unknown-unknown 
    wasm-bindgen --target web --no-typescript --out-dir $SRCDIR/static/ --out-name wasm $TARGET_DIR/$EXAMPLE.wasm
fi
