#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

CARGO_WEB_RELEASE=$(curl -L -s -H 'Accept: application/json' https://github.com/koute/cargo-web/releases/latest)
CARGO_WEB_VERSION=$(echo $CARGO_WEB_RELEASE | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')
CARGO_WEB_URL="https://github.com/koute/cargo-web/releases/download/$CARGO_WEB_VERSION/cargo-web-x86_64-unknown-linux-gnu.gz"

echo "Downloading cargo-web from: $CARGO_WEB_URL"
curl -L $CARGO_WEB_URL | gzip -d > cargo-web
chmod +x cargo-web

mkdir -p ~/.cargo/bin
mv cargo-web ~/.cargo/bin
