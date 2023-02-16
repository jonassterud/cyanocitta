window.onload = () => {
    try {
        load_and_display_metadata();
    }
    catch(error) {
        console.error(error);
    }
}

async function load_and_display_metadata() {
    const pk = await window.__TAURI__.invoke("get_my_pk");

    document.getElementById("picture_preview").classList.add(`${pk}_picture`);
    document.getElementById("picture").classList.add(`${pk}_picture`);
    document.getElementById("name").classList.add(`${pk}_name`);
    document.getElementById("display_name").classList.add(`${pk}_display_name`);
    document.getElementById("about").classList.add(`${pk}_about`);
    
    await window.__TAURI__.invoke("get_metadata", { pk: pk })
        .then((metadata) => {
            metadata = JSON.parse(metadata);
            document.getElementById("picture").value = metadata[pk].picture || ""
            
            display_metadata(metadata, pk);
        });
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