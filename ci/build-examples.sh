# Must be run from root of the repo:
# yew $ ./ci/build-examples.sh

output="$(pwd)/dist"
mkdir "$output"

for path in examples/*; do
  if [[ ! -d $path ]]; then
    continue
  fi

  example=$(basename "$path")

  # ssr does not need trunk
  if [[ "$example" == "simple_ssr" || "$example" == "ssr_router" ]]; then
    continue
  fi

  echo "building: $example"
  (
    # we are sure that $path exists
    # shellcheck disable=SC2164
    cd "$path"
    dist_dir="$output/$example"

    trunk build --release --dist "$dist_dir" --public-url "$PUBLIC_URL_PREFIX$example"
  )
done
