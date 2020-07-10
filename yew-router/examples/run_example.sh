#!/usr/bin/env bash

# The example to build.
EXAMPLE="${1%\/}"

# The page to start on.
INITIAL_URL="${2%\/}"

# Path to this script's parent dir
BASEDIR="$(dirname "$0")"

cd "$BASEDIR/$1"
(
    wasm-pack build \
        --dev \
        --target web \
        --out-dir ../_static/ \
        --out-name wasm \
        . \
    && cd ../_static \
    && python3 ../start_example_server.py "$INITIAL_URL"
)
