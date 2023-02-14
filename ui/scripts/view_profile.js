window.onload = () => {
    try {
        fill_profile_action_button();
        load_profile(5); // timeout???
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

async function load_profile(timeout) {
    let viewing_pk = window.localStorage.getItem("viewing_pk");
    let metadata = new Map();
    let notes = new Map();

    document.getElementById("profile_name").classList.add(`${viewing_pk}_name`);
    document.getElementById("profile_display_name").classList.add(`${viewing_pk}_display_name`);
    document.getElementById("profile_about").classList.add(`${viewing_pk}_about`);
    document.getElementById("profile_picture").classList.add(`${viewing_pk}_picture`);

    await window.__TAURI__.invoke("get_events_of", {
        filters: [{ authors: [viewing_pk], kinds: [0, 1, 2], limit: 5000 }],
        timeout: timeout,
    })
    .then((events) => JSON.parse(events))
    .then((events) => {
        events.forEach((event) => {
            switch (event.kind) {
                case 0:
                    metadata.set(event.pubkey, JSON.parse(event.content));
                    break;
                case 1:
                    notes.set(event.id, event);
                    break;
                case 2:
                    console.log(`recommended relay: ${event.content}`);
                    break;
            }
        });
    })
    .then(() => {
        notes.forEach((note) => {
            document.getElementById("notes").innerHTML += `
                <div class="note">
                    <img class="note_picture ${note.pubkey}_picture" src="media/avatar-default.svg">
                    <div>
                        <div>
                            <span class="note_display_name ${note.pubkey}_display_name">Display name</span>
                            <span class="note_name ${note.pubkey}_name">@Username</span>
                        </div>
                        <span class="note_content">${note.content}</span>
                    </div>
                </div>
            `;
        });
    })
    .then(() => {
        metadata.forEach((data, key) => {
            [...document.getElementsByClassName(`${key}_name`)].forEach((e) => e.innerHTML = data?.name || key);
            [...document.getElementsByClassName(`${key}_display_name`)].forEach((e) => e.innerHTML = data?.display_name || key.substring(0, 8) + "...");
            [...document.getElementsByClassName(`${key}_about`)].forEach((e) => e.innerHTML = data?.about || "");
            [...document.getElementsByClassName(`${key}_picture`)].forEach((e) => e.src = data?.picture || "media/avatar-default.svg");
        });
    })
    .catch((error) => {
        throw error;
    });
}