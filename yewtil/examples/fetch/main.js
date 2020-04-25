import init, { run_app } from './pkg/fetch.js';
async function main() {
  await init('./pkg/fetch_bg.wasm');
  run_app();
}
main()