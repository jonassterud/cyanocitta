window.onload = () => {
    try {
        unsubscribe_and_reset();
        display_profile_action_button();
        load_and_display_profile(5);
    }
    catch(error) {
        console.error(error);
    }
}

/**
 * Sets profile action button to "Follow" or "Edit profile".
 */
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

/**
 * Starts a loop that receives notes and then displays them.
 * 
 * @param {Number} timeout - the time to wait between updates
 */
async function load_and_display_profile(timeout) {
    const notes_el = document.getElementById("notes");
    const viewing_pk = window.localStorage.getItem("viewing_pk");

    document.getElementById("profile_name").classList.add(`${viewing_pk}_name`);
    document.getElementById("profile_display_name").classList.add(`${viewing_pk}_display_name`);
    document.getElementById("profile_about").classList.add(`${viewing_pk}_about`);
    document.getElementById("profile_picture").classList.add(`${viewing_pk}_picture`);

    // Subscribe to viewing_pk
    await window.__TAURI__.invoke("subscribe", {
        filters: [{
            kinds: [0, 1],
            authors: [viewing_pk],
            limit: 5000
        }]
    });

    // Increase note amount when scrolled to bottom
    let amount = 10;
    window.addEventListener("scroll", () => {
        if ((window.innerHeight + window.scrollY) >= document.body.offsetHeight) {
            amount += 10;
        }
    });

    // Loop to get received notes and display them
    while (true) {
        await window.__TAURI__.invoke("get_received_notes", {
            sort_by_date: true,
            amount: amount
        })
            .then((notes) => {
                notes = JSON.parse(notes);
                notes_el.innerHTML = get_notes_html(notes);
            });

        await window.__TAURI__.invoke("get_metadata")
            .then((metadata) => {
                metadata = JSON.parse(metadata);
                display_metadata(metadata, [viewing_pk]);
            });

        await new Promise((resolve) => setTimeout(resolve, 1000 * timeout));
    }
}