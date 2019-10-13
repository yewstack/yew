#!/usr/bin/env bash
set -euxo pipefail

echo "$(rustup default)" | grep -q "stable"
if [ "$?" = "0" ]; then
  cargo fmt --all -- --check
  cargo clippy -- --deny=warnings
fi
