#!/usr/bin/env bash
set -e
shopt -s extglob

CARGO="cargo +nightly"
WASM_BINDGEN=~/.cache/trunk/"$(cargo tree --package wasm-bindgen --depth=0 --format="{p}" | sed -e 's/ v/-/g')/wasm-bindgen"
WASM_OPT="$(ls -dv1 ~/.cache/trunk/wasm-opt-version_* | tail -n 1)"/bin/wasm-opt #  will select most recently installed :)

PROFILE="release"
THIS_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
TARGET_DIR=$(cd -- "$THIS_DIR"/../../target/ &> /dev/null && pwd)
OPT=1

RUSTFLAGS="-Clink-args=--emit-relocs$(case "$CARGO" in *nightly*) echo " -Zunstable-options -Cpanic=immediate-abort" ;; esac)" \
    $CARGO build --target wasm32-unknown-unknown \
    $(case $PROFILE in "debug") ;; "release") echo "--release" ;; *) echo '--profile "${PROFILE}"' ;; esac)

mkdir -p dist/
GLOBIGNORE=".:.."
rm -rf dist/*
mkdir dist/.stage
(
  wasm_split_cli --verbose "$TARGET_DIR/wasm32-unknown-unknown/${PROFILE}/split-wasm.wasm" "$THIS_DIR"/dist/.stage/ \
    > "$THIS_DIR"/dist/.stage/split.log
)
echo "running wasm-bindgen"
$WASM_BINDGEN dist/.stage/main.wasm --out-dir dist/.stage --no-demangle --target web --keep-lld-exports --no-typescript
if [ "$OPT" == 1 ] ; then
  echo "running wasm-opt"
  for wasm in dist/.stage/!(main).wasm ; do
    $WASM_OPT -Os "$wasm" -o dist/"$(basename -- "$wasm")"
  done
else
  for wasm in dist/.stage/!(main).wasm ; do
    mv "$wasm" dist/"$(basename -- "$wasm")"
  done
fi
echo "moving to dist dir"
mv dist/.stage/*.!(wasm) dist
#rmdir dist/.stage
cp index.html dist/

