function set_viewing_pk_to_my_pk() {
    window.__TAURI__.invoke("get_my_pk")
        .then((my_pk) => {
            window.localStorage.setItem("viewing_pk", my_pk);
        })
        .catch((error) => {
            console.error(error);
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
                        <span class="note_display_name ${notes[key].pubkey}_display_name">Display name</span>
                        <span class="note_name ${notes[key].pubkey}_name">@Username</span>
                    </div>
                    <span class="note_content">${notes[key].content}</span>
                </div>
            </div>
        `;
    }

    return out;
}

function display_metadata(metadata) {
    for (let key in metadata) {
        [...document.getElementsByClassName(`${key}_name`)].forEach((e) => e.innerHTML = metadata[key].name || key);
        [...document.getElementsByClassName(`${key}_display_name`)].forEach((e) => e.innerHTML = metadata[key].display_name || key.substring(0, 8) + "...");
        [...document.getElementsByClassName(`${key}_about`)].forEach((e) => e.innerHTML = metadata[key].about || "");
        [...document.getElementsByClassName(`${key}_picture`)].forEach((e) => e.src = metadata[key].picture || "media/avatar-default.svg");
    }
}