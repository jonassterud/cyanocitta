window.onload = () => {
    try {
        display_profile_action_button();
        display_profile(5);
    }
    catch(error) {
        console.error(error);
    }
}

async function display_profile_action_button() {
    const profile_action_button_el = document.getElementById("profile_action_button");
    const viewing_pk = window.localStorage.getItem("viewing_pk");
    const my_pk = await window.__TAURI__.invoke("get_my_pk");
    
    if (viewing_pk === my_pk) {
        profile_action_button_el.innerHTML = "Edit profile";
        profile_action_button_el.href = "edit_profile.html";
    } else {
        profile_action_button_el.innerHTML = "Follow";
    }
}

async function display_profile(timeout) {
    const viewing_pk = window.localStorage.getItem("viewing_pk");

    document.getElementById("profile_name").classList.add(`${viewing_pk}_name`);
    document.getElementById("profile_display_name").classList.add(`${viewing_pk}_display_name`);
    document.getElementById("profile_about").classList.add(`${viewing_pk}_about`);
    document.getElementById("profile_picture").classList.add(`${viewing_pk}_picture`);

    await window.__TAURI__.invoke("req_events_of", {
        filters: [{ authors: [viewing_pk], kinds: [0, 1, 2], limit: 5000 }]
    });

    while (true) {
        await window.__TAURI__.invoke("get_received_notes", { pk: viewing_pk })
            .then((notes) => {
                notes = JSON.parse(notes);
                document.getElementById("notes").innerHTML = get_notes_html(notes);
            });

        await window.__TAURI__.invoke("get_metadata", { pk: viewing_pk })
            .then((metadata) => {
                metadata = JSON.parse(metadata);
                display_metadata(metadata);
            });

        await new Promise((resolve) => setTimeout(resolve, 1000 * timeout));
    }
}