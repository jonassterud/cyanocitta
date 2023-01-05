var app_data;

async function load_app_data() {
    const invoke = window.__TAURI__.invoke;

    let data = await invoke("get_app_data");
    app_data = JSON.parse(data);
}