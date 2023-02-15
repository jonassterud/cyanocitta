window.onload = () => {
    try {
        fill_profile_action_button();
        load_profile(5);
    }
    catch(error) {
        console.error(error);
    }
}

async function fill_profile_action_button() {
    let profile_action_button = document.getElementById("profile_action_button");
    let viewing_pk = window.localStorage.getItem("viewing_pk");
    let my_pk = await window.__TAURI__.invoke("get_my_pk");
    
    if (viewing_pk === my_pk) {
        profile_action_button.innerHTML = "Edit profile";
        profile_action_button.href = "edit_profile.html";
    } else {
        profile_action_button.innerHTML = "Follow";
    }
}

function display_notes(notes) {
    document.getElementById("notes").innerHTML = ""; // optimize later

    for (let key in notes) {
        document.getElementById("notes").innerHTML += `
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
}

function display_metadata(metadata) {
    for (let key in metadata) {
        [...document.getElementsByClassName(`${key}_name`)].forEach((e) => e.innerHTML = metadata[key].name || key);
        [...document.getElementsByClassName(`${key}_display_name`)].forEach((e) => e.innerHTML = metadata[key].display_name || key.substring(0, 8) + "...");
        [...document.getElementsByClassName(`${key}_about`)].forEach((e) => e.innerHTML = metadata[key].about || "");
        [...document.getElementsByClassName(`${key}_picture`)].forEach((e) => e.src = metadata[key].picture || "media/avatar-default.svg");
    }
}

async function load_profile(timeout) {
    let viewing_pk = window.localStorage.getItem("viewing_pk");

    document.getElementById("profile_name").classList.add(`${viewing_pk}_name`);
    document.getElementById("profile_display_name").classList.add(`${viewing_pk}_display_name`);
    document.getElementById("profile_about").classList.add(`${viewing_pk}_about`);
    document.getElementById("profile_picture").classList.add(`${viewing_pk}_picture`);

    await window.__TAURI__.invoke("req_events_of", {
        filters: [{ authors: [viewing_pk], kinds: [0, 1, 2], limit: 5000 }]
    });

    setInterval(async function() {
        await window.__TAURI__.invoke("get_received_notes", { pk: viewing_pk })
            .then((notes) => {
                notes = JSON.parse(notes);
                display_notes(notes);
            })
            .catch((error) => {
                throw error;
            });

        await window.__TAURI__.invoke("get_metadata", { pk: viewing_pk })
            .then((metadata) => {
                metadata = JSON.parse(metadata);
                display_metadata(metadata);
            })
            .catch((error) => {
                throw error;
            });
    }(), timeout * 1000);
}