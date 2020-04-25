import init, { run_app } from './pkg/futures.js';
async function main() {
   await init('./pkg/futures_bg.wasm');
   run_app();
}
main()