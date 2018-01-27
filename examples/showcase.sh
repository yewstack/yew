trap ctrl_c INT

PID=-1

function ctrl_c() {
    echo "** Killing the demo..."
    kill $PID
}

for example in */ ; do
    cd $example
    cargo web start --target-webasm-emscripten &
    PID=$!
    wait $PID
    cd ..
done
