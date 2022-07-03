if [ ! -f "Cargo.lock" ]; then
  cargo fetch
fi

VERSION=$(cargo pkgid --frozen wasm-bindgen | cut -d ":" -f 3)

# Cargo decided to change syntax after 1.61
if [ "$VERSION" = "" ]; then
  VERSION=$(cargo pkgid --frozen wasm-bindgen | cut -d "@" -f 2)
fi

cargo +stable install --version $VERSION wasm-bindgen-cli
