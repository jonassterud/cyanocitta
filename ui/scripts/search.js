window.onload = () => {
    try {
        unsubscribe_and_reset();
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
    await window.__TAURI__.invoke("subscribe", {
        filters: [{
            kinds: [1],
            search: search_input_el.value,
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
            amount: amount,
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

        await new Promise((resolve) => setTimeout(resolve, 1000 * timeout));
    }
}