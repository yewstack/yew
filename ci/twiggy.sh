#!/bin/bash
cargo build --all --release --target wasm32-unknown-unknown
cd master/yew
cargo build --all --release --target wasm32-unknown-unknown
cd ../../
mkdir twiggy_results
for filename in target/wasm32-unknown-unknown/release/*.wasm; do
  localname=$(basename $filename)
  echo "Comparing old $localname against new $localname"
  touch twiggy_results/$localname.csv
  twiggy diff master/yew/$filename $filename --format csv -o twiggy_results/$localname.csv -n 10000
done
cd twiggy-analysis
cargo build --release
cd ../
./twiggy-analysis/target/release/twiggy-analysis
