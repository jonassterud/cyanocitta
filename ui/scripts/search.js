window.onload = async () => {
    try {
        //
    }
    catch(error) {
        console.error(error);
    }
}

/**
 * Get notes matching search term and then display them.
 * Starts a loop for updating metadata.
 */
async function search_and_display() {
    const search_input_el = document.getElementById("search_input");
    const notes_el = document.getElementById("notes");
    const timeout = 5;

    // Subscribe to search input
    let subscription_id = await window.__TAURI__.invoke("subscribe", {
        filters: [{
            kinds: [1],
            search: search_input_el.value,
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
            .then(async (notes) => {
                notes = JSON.parse(notes);
                notes_el.innerHTML = get_notes_html(notes);
    
                // Subscribe to metadata of note authors
                await window.__TAURI__.invoke("subscribe", {
                    filters: [{
                        kinds: [0],
                        authors: notes.map((n) => n.pubkey),
                        limit: 5000
                    }]
                });
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