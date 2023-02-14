window.onload = () => {
    try {
        fill_profile_action_button();
        load_profile(5); // timeout???
    }
    catch(error) {
        console.error(error);
    }
}

function fill_profile_action_button() {
    let pk = window.localStorage.getItem("pk");

    if (pk === null) {
        throw Error("missing public key");
    }
    
    window.__TAURI__.invoke("get_pk")
    .then((self_pk) => {
        let profile_action_button = document.getElementById("profile_action_button");

        if (pk === self_pk) {
            profile_action_button.innerHTML = "Edit profile";
            profile_action_button.href = "edit_profile.html";
            
        } else {
            profile_action_button.innerHTML = "Follow";
        }
    })
    .catch((error) => {
        throw error;
    });
}

function load_profile(timeout) {
    let pk = window.localStorage.getItem("pk");
    let metadatas = new Map();
    let notes = new Map();

    if (pk === null) {
        throw Error("missing public key");
    }

    document.getElementById("profile_name").classList.add(`${pk}_name`);
    document.getElementById("profile_display_name").classList.add(`${pk}_display_name`);
    document.getElementById("profile_about").classList.add(`${pk}_about`);
    document.getElementById("profile_picture").classList.add(`${pk}_picture`);

    window.__TAURI__.invoke("get_events_of", {
        filters: [{ authors: [pk], kinds: [0, 1, 2], limit: 5000 }],
        timeout: timeout,
    })
    .then((events) => JSON.parse(events))
    .then((events) => {
        events.forEach((event) => {
            switch (event.kind) {
                case 0:
                    metadatas.set(event.pubkey, JSON.parse(event.content));

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
        metadatas.forEach((metadata, key) => {
            [...document.getElementsByClassName(`${key}_name`)].forEach((e) => e.innerHTML = metadata?.name || key);
            [...document.getElementsByClassName(`${key}_display_name`)].forEach((e) => e.innerHTML = metadata?.display_name || key.substring(0, 8) + "...");
            [...document.getElementsByClassName(`${key}_about`)].forEach((e) => e.innerHTML = metadata?.about || "");
            [...document.getElementsByClassName(`${key}_picture`)].forEach((e) => e.src = metadata?.picture || "media/avatar-default.svg");
        });
    })
    .catch((error) => {
        throw error;
    });
}