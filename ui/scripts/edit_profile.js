window.onload = () => {
    try {
        load_metadata();
    }
    catch(error) {
        console.error(error);
    }
}

function _get_metadata() {
    return window.__TAURI__.invoke("get_metadata")
        .then((response) => {
            return JSON.parse(response);
        });
}

function _set_metadata(metadata) {
    return window.__TAURI__.invoke("set_metadata", { metadata: JSON.stringify(metadata) });
}

async function load_metadata() {
    let metadata = await _get_metadata();
    
    document.getElementById("picture").value = metadata.picture || "";
    document.getElementById("name").value = metadata.name || "";
    document.getElementById("display_name").value = metadata.display_name || "";
    document.getElementById("about").value = metadata.about || "";
}

async function save_metadata() {
    let metadata = {
        picture: document.getElementById("picture").value || "",
        name: document.getElementById("name").value || "",
        display_name: document.getElementById("display_name").value || "",
        about: document.getElementById("about").value || "",
    };

    await _set_metadata(metadata);
}