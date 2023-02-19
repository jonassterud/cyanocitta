window.onload = async () => {
    try {
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
    const profile_button_container_el = document.getElementById("profile_button_container");
    const viewing_pk = window.localStorage.getItem("viewing_pk");
    const my_pk = await window.__TAURI__.invoke("get_my_pk");
    
    if (viewing_pk === my_pk) {
        profile_button_container_el.innerHTML += `
            <a class="button" href="edit_profile.html">Edit profile</a>
        `;
    } else {
        let is_following = await window.__TAURI__.invoke("is_following", { pk: viewing_pk })
            .then((resp) => JSON.parse(resp));
        
        if (is_following) {
            profile_button_container_el.innerHTML = `
                <button class="button" onclick="follow('${viewing_pk}').then(display_profile_action_button)">Unfollow</button>
            `;
        } else {
            profile_button_container_el.innerHTML = `
                <button class="button" onclick="follow('${viewing_pk}').then(display_profile_action_button)">Follow</button>
            `;
        }       
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
    let subscription_id = await window.__TAURI__.invoke("subscribe", {
        filters: [{
            kinds: [0, 1],
            authors: [viewing_pk],
            limit: 5000
        }]
    }).then((resp) => JSON.parse(resp));

    // Unsubscribe on unload
    window.addEventListener("beforeunload", async () => {
        await window.__TAURI__.invoke("unsubscribe", { subscriptionId: subscription_id });
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
        try {
            await window.__TAURI__.invoke("get_received_notes", {
                subscriptionId: subscription_id,
                amount: amount,
                sort: true,
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
        }
        catch(error) {
            console.error(error);
        }

        await new Promise((resolve) => setTimeout(resolve, 1000 * timeout));
    }
}