save_and_exit_on_close();

/**
 * Adds a listener for window close requests that saves state before exiting.
 */
function save_and_exit_on_close() {
    window.__TAURI__.window.appWindow.once(window.__TAURI__.event.TauriEvent.WINDOW_CLOSE_REQUESTED, async function() {
        await window.__TAURI__.invoke("save_and_exit");
        await window.__TAURI__.window.appWindow.close();
    });
}

/**
 * Unsubscribes.
 */
async function unsubscribe() {
    await window.__TAURI__.invoke("unsubscribe");
}

/**
 * Sets the `viewing_pk` variable in local storage to this clients public key.
 */
async function set_viewing_pk_to_my_pk() {
    await window.__TAURI__.invoke("get_my_pk")
        .then((my_pk) => {
            window.localStorage.setItem("viewing_pk", my_pk);
        });
}

/**
 * Sets the `viewing_pk` variable in local storage to the given parameter.
 * 
 * @param {String} pk - public key in hex format.
 */
async function set_viewing_pk(pk) {
    window.localStorage.setItem("viewing_pk", pk);
}

/**
 * Generates HTML for notes.
 * 
 * @param {Array<[String, Object]>} notes 
 * @returns {String} HTML string
 */
function get_notes_html(notes) {
    let out = "";

    notes.forEach((note) => {
        const pk = note.pubkey;
        const name = pk.substr(0, 5) + "...";
        const display_name = name;
        const picture = "media/avatar-default.svg";
        const date = new Date(Date.now() - note.created_at).getHours() + "h";
        out += `
            <div class="note">
                <a class="clickable" href="view_profile.html" onclick="set_viewing_pk('${pk}')">
                    <img class="note_picture ${pk}_picture" src="${picture}">
                </a>
                <div>
                    <div>
                        <span class="note_display_name ${pk}_display_name">${display_name}</span>
                        <span class="note_name ${pk}_name">${name}</span>
                        <span class="note_date">${date}</span>
                    </div>
                    <span class="note_content">${note.content}</span>
                </div>
            </div>
        `;
    });

    return out;
}

/**
 * Loops trough metadata and inserts data into matching HTML elements.
 * 
 * @param {Object} metadata - metadata object
 * @param {Array<String>} pks - array of public keys
 */
function display_metadata(metadata, pks=null) {
    const keys = pks ? pks : Object.keys(metadata);
    keys.forEach((key) => {
        const name = metadata[key]?.name || key;
        const display_name = metadata[key]?.display_name || name;
        const about = metadata[key]?.about || "";
        const picture = metadata[key]?.picture || "media/avatar-default.svg";

        [...document.getElementsByClassName(`${key}_name`)].forEach((e) => e.innerHTML = e.value = name);
        [...document.getElementsByClassName(`${key}_display_name`)].forEach((e) => e.innerHTML = e.value = display_name);
        [...document.getElementsByClassName(`${key}_about`)].forEach((e) => e.innerHTML = e.value = about);
        [...document.getElementsByClassName(`${key}_picture`)].forEach((e) => e.innerHTML = e.value = e.src = picture);
    });
}