import * as wasm from "nebula-wasm-app";

window.addEventListener('load', async () => {
    await init('./pkg/websockets_bg.wasm');
});