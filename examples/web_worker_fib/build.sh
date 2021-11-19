#!/usr/bin/env bash
if ! command -v wasm-pack 2>&1 >/dev/null;
then
	echo 'error: you must install wasm-pack'
	exit
fi

wasm-pack build --target no-modules --out-name wasm --out-dir ./static --no-typescript
