window.onload = async () => {
    try {
        load_and_display_home(5);
    }
    catch(error) {
        console.error(error);
    }
}

/**
 * Starts a loop that receives notes and then displays them.
 * 
 * @param {Number} timeout - the time to wait between updates
 */
async function load_and_display_home(timeout) {
    const notes_el = document.getElementById("notes");

    // Subscribe to following
    let following = await window.__TAURI__.invoke("get_following").then((resp) => JSON.parse(resp));
    let existing_subscription = window.localStorage.getItem("home_subscription");
    let subscription_id = await window.__TAURI__.invoke("subscribe", {
        filters: [{
            kinds: [0, 1, 2],
            authors: following,
            limit: 5000,
        }]
    }).then((resp) => JSON.parse(resp));

    // Unsubscribe existing subscription if necessary
    if (existing_subscription != null && existing_subscription != subscription_id) {
        await window.__TAURI__.invoke("unsubscribe", { subscriptionId: existing_subscription });
        window.localStorage.setItem("home_subscription", subscription_id);
    }

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
                    display_metadata(metadata);
                });
        }
        catch(error) {
            console.error(error);
        }

        await new Promise((resolve) => setTimeout(resolve, 1000 * timeout));
    }
}