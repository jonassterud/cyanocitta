window.onload = () => {
    try {
        load_metadata();
    }
    catch(error) {
        console.error(error);
    }
}

function load_metadata() {
    window.__TAURI__.invoke("get_metadata")
    .then((metadata) => JSON.parse(metadata))
    .then((metadata) => {
        document.getElementById("picture_preview").src = metadata.picture || "";
        document.getElementById("picture").value = metadata.picture || "";
        document.getElementById("name").value = metadata.name || "";
        document.getElementById("display_name").value = metadata.display_name || "";
        document.getElementById("about").value = metadata.about || "";
    })
    .catch((error) => {
        throw error;
    });
}

async function save_metadata() {
    window.__TAURI__.invoke("set_metadata", {
        metadata: JSON.stringify({
            picture: document.getElementById("picture").value || "",
            name: document.getElementById("name").value || "",
            display_name: document.getElementById("display_name").value || "",
            about: document.getElementById("about").value || "",
        })
    })
    .catch((error) => {
        throw error;
    });
}