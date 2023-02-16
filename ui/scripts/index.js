window.onload = () => {
    try {
        display_home(5);
    }
    catch(error) {
        console.error(error);
    }
}

async function display_home(timeout) {
    const notes_el = document.getElementById("notes");

    while (true) {
        await window.__TAURI__.invoke("get_received_notes")
            .then((notes) => {
                notes = JSON.parse(notes);
                notes_el.innerHTML = get_notes_html(notes);
            });

        await window.__TAURI__.invoke("get_metadata")
            .then((metadata) => {
                metadata = JSON.parse(metadata);
                display_metadata(metadata);
            });

        await new Promise((resolve) => setTimeout(resolve, 1000 * timeout));
    }
}