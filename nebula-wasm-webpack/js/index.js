// function onSourceClose(_) {
//     console.log('media source closed');
// }
// function onSourceOpen(_) {
//     console.log('media source opened');
// }

// mediaSource.addEventListener('sourceclose', onSourceClose);
// mediaSource.addEventListener('sourceopen', onSourceOpen);
import("../pkg/index.js").then((wasm) => {
    let mediaSource = new MediaSource();
    var video = document.querySelector('video');
    video.src = URL.createObjectURL(mediaSource);
    console.log('starting...');
    wasm.app(mediaSource);
}).catch(console.error);

