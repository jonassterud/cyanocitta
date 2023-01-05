var invoke; // Tauri

window.onload = async () => {
    // Load Tauri APIs
    invoke = window.__TAURI__.invoke;

    load().catch((error) => {
        console.error(error);
    });
}

async function load() {
    // Load app data
    await load_app_data();

    // ...
}