const invoke = window.__TAURI__.invoke;

window.onload = function () {
    try {
        invoke("try_listen_all_relays", { buffer: 100 });
    } catch (err) {
        console.error(err);
    }
};
