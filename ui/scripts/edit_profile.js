window.onload = () => {
    try {
        load_metadata();
    }
    catch(error) {
        console.error(error);
    }
}

async function load_metadata() {
    const pk = await window.__TAURI__.invoke("get_my_pk");
    const resp = await window.__TAURI__.invoke("get_metadata", { pk: pk });
    const metadata = JSON.parse(resp);

    document.getElementById("picture_preview").src = metadata.picture || "";
    document.getElementById("picture").value = metadata.picture || "";
    document.getElementById("name").value = metadata.name || "";
    document.getElementById("display_name").value = metadata.display_name || "";
    document.getElementById("about").value = metadata.about || "";
}

async function save_metadata() {
    const metadata = {
        picture: document.getElementById("picture").value || "",
        name: document.getElementById("name").value || "",
        display_name: document.getElementById("display_name").value || "",
        about: document.getElementById("about").value || "",
    };

    await window.__TAURI__.invoke("set_metadata", { metadata: metadata });
}