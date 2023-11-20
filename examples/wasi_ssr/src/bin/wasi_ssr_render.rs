fn main() {
    println!("Current dir: {:?}", std::env::current_dir());
    println!("Current file: {:?}", std::env::current_exe());

    // TODO - Run the WASI module here by wasmtime_wasi::preview2
}
