save_and_exit_on_close();

function save_and_exit_on_close() {
    window.__TAURI__.window.appWindow.once(window.__TAURI__.event.TauriEvent.WINDOW_CLOSE_REQUESTED, async function() {
        await window.__TAURI__.invoke("save_and_exit");
        await window.__TAURI__.window.appWindow.close();
    });
}

async function set_viewing_pk_to_my_pk() {
    await window.__TAURI__.invoke("get_my_pk")
        .then((my_pk) => {
            window.localStorage.setItem("viewing_pk", my_pk);
        });
}

function get_notes_html(notes) {
    let out = "";

    for (let key in notes) {
        out += `
            <div class="note">
                <img class="note_picture ${notes[key].pubkey}_picture" src="media/avatar-default.svg">
                <div>
                    <div>
                        <span class="note_display_name ${notes[key].pubkey}_display_name">${key.substring(0, 8) + "..."}</span>
                        <span class="note_name ${notes[key].pubkey}_name">${key}</span>
                    </div>
                    <span class="note_content">${notes[key].content}</span>
                </div>
            </div>
        `;
    }

    return out;
}

function display_metadata(metadata, pk=null) {
    function update(key, metadata) {
        [...document.getElementsByClassName(`${key}_name`)].forEach((e) => e.innerHTML = e.value = metadata[key]?.name || key);
        [...document.getElementsByClassName(`${key}_display_name`)].forEach((e) => e.innerHTML = e.value = metadata[key]?.display_name || key.substring(0, 8) + "...");
        [...document.getElementsByClassName(`${key}_about`)].forEach((e) => e.innerHTML = e.value = metadata[key]?.about || "");
        [...document.getElementsByClassName(`${key}_picture`)].forEach((e) => e.innerHTML = e.value = e.src = metadata[key]?.picture || "media/avatar-default.svg");
    }

    if (pk === null) {
        for (let key in metadata) {
            update(key, metadata);
        }
    } else {
        update(pk, metadata);
    }
}