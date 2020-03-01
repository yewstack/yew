#!/usr/bin/env bash

echo "$(rustup default)" | grep -q "stable"
if [ "$?" != "0" ]; then
  # only run checks on stable
  exit 0
fi

set -euxo pipefail
cargo fmt --all -- --check
cargo clippy --features std_web -- --deny=warnings
cargo clippy --features web_sys -- --deny=warnings
