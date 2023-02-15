window.onload = () => {
    try {
        save_state_on_close();
        load(5);
    }
    catch(error) {
        console.error(error);
    }
}

async function load(timeout) {
    setInterval(async function() {
        await window.__TAURI__.invoke("get_received_notes")
            .then((notes) => {
                notes = JSON.parse(notes);
                document.getElementById("notes").innerHTML = get_notes_html(notes);
            })
            .catch((error) => {
                throw error;
            });

        await window.__TAURI__.invoke("get_metadata")
            .then((metadata) => {
                metadata = JSON.parse(metadata);
                display_metadata(metadata);
            })
            .catch((error) => {
                throw error;
            });
    }(), timeout * 1000);
}