PID=-1

function ctrl_c() {
    echo "** Killing the demo..."
    kill $PID
}

for example in */ ; do
    cd $example
    cargo update
    cargo web build --target wasm32-unknown-emscripten
    cd ..
done

trap ctrl_c INT

for example in */ ; do
    cd $example
    cargo web start --target wasm32-unknown-emscripten &
    PID=$!
    wait $PID
    cd ..
done
