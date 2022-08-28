# Must be run from root of the repo:
# yew $ ./ci/build-examples.sh

output="$(pwd)/dist"
mkdir -p "$output"

failure=false
for path in examples/*; do
  if [[ ! -d $path ]]; then
    continue
  fi

  example=$(basename "$path")

  # ssr does not need trunk
  if [[ "$example" == "simple_ssr" || "$example" == "ssr_router" ]]; then
    continue
  fi

  echo "::group::Building $example"
  if ! (
    set -e
    # we are sure that $path exists
    # shellcheck disable=SC2164
    cd "$path"
    dist_dir="$output/$example"
    export RUSTFLAGS="--cfg nightly_yew"
    if [[ "$example" == "boids" || "$example" == "password_strength" ]]; then
      # works around issue rust-lang/rust#96486
      # where the compiler forgets to link some symbols connected to const_eval
      # only an issue on nightly and with build-std enabled which we do for code size
      # this deoptimizes only the examples that otherwise fail to build
      export RUSTFLAGS="-Zshare-generics=n -Clto=thin $RUSTFLAGS"
    fi

    trunk build --release --dist "$dist_dir" --public-url "$PUBLIC_URL_PREFIX$example"

    # check that there are no undefined symbols. Those generate an import .. from 'env',
    # which isn't available in the browser.
    { cat "$dist_dir"/*.js | grep -q -e "from 'env'" ; } && exit 1 || true
  ) ; then
    echo "::error ::$example failed to build"
    failure=true
  fi
  echo "::endgroup::"
done
if [ "$failure" = true ] ; then
    exit 1
fi
