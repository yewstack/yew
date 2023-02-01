#!/usr/bin/env bash
set -xe

# You can extract the feature list with the following command:
# cargo hack check --feature-powerset --exclude-features nightly

# You need to run this script in packages/yew

cargo clippy --no-default-features -- --deny=warnings
cargo clippy --no-default-features --features csr -- --deny=warnings
cargo clippy --no-default-features --features default -- --deny=warnings
cargo clippy --no-default-features --features csr,default -- --deny=warnings
cargo clippy --no-default-features --features hydration -- --deny=warnings
cargo clippy --no-default-features --features default,hydration -- --deny=warnings
cargo clippy --no-default-features --features ssr -- --deny=warnings
cargo clippy --no-default-features --features csr,ssr -- --deny=warnings
cargo clippy --no-default-features --features default,ssr -- --deny=warnings
cargo clippy --no-default-features --features csr,default,ssr -- --deny=warnings
cargo clippy --no-default-features --features hydration,ssr -- --deny=warnings
cargo clippy --no-default-features --features default,hydration,ssr -- --deny=warnings
