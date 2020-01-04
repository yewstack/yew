#!/usr/bin/env bash

PID=-1

function ctrl_c() {
    echo "** Killing the demo..."
    kill $PID
}

function build_std_web() {
    for example in */ ; do
        if [[ $example == server* ]]; then
            continue
        fi
        if [[ $example == static* ]]; then
            continue
        fi
        echo "Building: $example"
        cd $example
        cargo update
        cargo web build --target wasm32-unknown-unknown
        cd ..
    done
}

function build_web_sys() {
    for example in */ ; do
        if [[ $example == server* ]]; then
            continue
        fi
        if [[ $example == static* ]]; then
            continue
        fi
        echo "Building: $example"
        cd $example
        cargo update
        cargo build --target wasm32-unknown-unknown
        wasm-bindgen --target web --no-typescript --out-dir ../static/ --out-name wasm ../target/wasm32-unknown-unknown/debug/$example.wasm
        cd ..
    done
}

function run_std_web() {
    trap ctrl_c INT
    for example in */ ; do
        if [[ $example == server* ]]; then
            continue
        fi
        if [[ $example == static* ]]; then
            continue
        fi
        echo "Running: $example"
        cd $example
        cargo web start --target wasm32-unknown-unknown &
        PID=$!
        wait $PID
        cd ..
    done
}

function run_web_sys() {
    trap ctrl_c INT
    for example in */ ; do
        if [[ $example == server* ]]; then
            continue
        fi
        if [[ $example == static* ]]; then
            continue
        fi
        echo "Running: $example"
        cd $example
        cargo build --target wasm32-unknown-unknown
        wasm-bindgen --target web --no-typescript --out-dir ../static/ --out-name wasm ../target/wasm32-unknown-unknown/debug/$example.wasm
        http -r ../static/
        PID=$!
        wait $PID
        cd ..
    done
}

function clean() {
    trap ctrl_c INT
    for example in */ ; do
        echo "Cleaning: $example"
        cd $example
        cargo clean
        PID=$!
        wait $PID
        cd ..
    done
}

case "$1" in
    --help)
        echo "Available commands: build, run, clean"
    ;;
    build_std_web)
        build_std_web
    ;;
    build_web_sys)
        build_web_sys
    ;;
    run_std_web)
        run_std_web
    ;;
    run_web_sys)
        run_web_sys
    ;;
    clean)
        clean
    ;;
esac
