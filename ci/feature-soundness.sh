#!/usr/bin/env bash
set -xe

# You can extract the feature list with the following command:
# cargo hack -p yew -p yew-agent -p yew-router --feature-powerset --print-command-list clippy -- -D warnings

cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features default,hydration,ssr -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features csr -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features default -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features csr,default -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features hydration -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features default,hydration -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features ssr -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features csr,ssr -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features default,ssr -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features csr,default,ssr -- -D warnings
cargo clippy --manifest-path packages/yew/Cargo.toml --no-default-features --features hydration,ssr -- -D warnings
cargo clippy --manifest-path packages/yew-agent/Cargo.toml -- -D warnings
cargo clippy --manifest-path packages/yew-router/Cargo.toml -- -D warnings
