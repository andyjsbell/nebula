import("../pkg/index.js").then(wasm => {
    let state = wasm.State.new();
    let mediaSource = new MediaSource();
    var video = document.querySelector('video');
    video.src = URL.createObjectURL(mediaSource);
    let mediaSourceLoaded = false;
    mediaSource.addEventListener('sourceopen', function(_) {
        console.log('media source opened');
        mediaSourceLoaded = true;
    });

    mediaSource.addEventListener('sourceclose', function(_) {
        console.log('media source closed');
        mediaSourceLoaded = false;
    });

    let socketURL = 'ws://localhost:9001/socket';
    var ws = new WebSocket(socketURL);
    ws.binaryType = 'arraybuffer';
    ws.addEventListener('open', function(event) {
        wasm.request_new_frame(ws);
    });

    ws.addEventListener('message',function(event) {
        console.log('data over web socket');
        if (mediaSourceLoaded) {
            wasm.process_packet(event.data, mediaSource, state);
            wasm.request_new_frame(ws);
        }
    });
    ws.addEventListener('error', function(e) {
        console.log('Socket Error');
    });

    let timerId = setInterval(function() {
        if (mediaSourceLoaded)
            wasm.write_to_buffer(state);
    }, 50);

});
