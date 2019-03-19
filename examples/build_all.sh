#!/usr/bin/env bash

PID=-1

function ctrl_c() {
    echo "** Killing the demo..."
    kill $PID
}

function build() {
    for example in */ ; do
        if [[ $example == server* ]]; then
            continue
        fi
        echo "Building: $example"
        cd $example
        cargo update
        cargo web build --target wasm32-unknown-unknown
        cd ..
    done
}

function run() {
    trap ctrl_c INT
    for example in */ ; do
        if [[ $example == server* ]]; then
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
    build)
        build
    ;;
    run)
        run
    ;;
    clean)
        clean
    ;;
    *)
        build
    ;;
esac
