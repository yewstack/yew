trap ctrl_c INT

PID=-1

function ctrl_c() {
    echo "** Killing the demo..."
    kill $PID
}

for example in */ ; do
    cd $example
    cargo update
    cargo web start --target wasm32-unknown-emscripten &
    PID=$!
    wait $PID
    cd ..
done
