#!/usr/bin/env bash
echo "$(rustup default)" | grep -q "1.39.0"
emscripten_supported=$?
set -euxo pipefail # https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/

pushd yew

# Some examples are known not to work with some builds, i.e. futures with the
# std_web feature.  Other items in the examples/ directory are helpers, i.e.
# pub_sub and server.  These block lists allow us to exempt some examples but
# default opt-in any new examples to CI testing.
COMMON_SKIP_EXAMPLES="examples/static examples/pub_sub examples/server \
  examples/target examples/web_sys examples/std_web"
STD_WEB_SKIP_EXAMPLES="${COMMON_SKIP_EXAMPLES:?} examples/futures"
WEB_SYS_SKIP_EXAMPLES="${COMMON_SKIP_EXAMPLES:?}"

# Make sure all examples are buildable with stdweb and web-sys.
for ex in $(find examples -maxdepth 1 -mindepth 1 -type d); do
  pushd $ex

  # TODO Can't build some demos with release, need fix

  if [ "$emscripten_supported" == "0" ]; then
    if [[ ! " ${STD_WEB_SKIP_EXAMPLES[@]} " =~ " ${ex} " ]]; then
      # TODO - Emscripten builds are broken on rustc > 1.39.0
      cargo web build --target asmjs-unknown-emscripten --features std_web
      cargo web build --target wasm32-unknown-emscripten --features std_web
    fi
  fi

  if [[ ! " ${STD_WEB_SKIP_EXAMPLES[@]} " =~ " ${ex} " ]]; then
    cargo web build --target wasm32-unknown-unknown --features std_web
  fi
  if [[ ! " ${WEB_SYS_SKIP_EXAMPLES[@]} " =~ " ${ex} " ]]; then
    cargo build --target wasm32-unknown-unknown --features web_sys
  fi

  # Reset cwd
  popd
done
popd
