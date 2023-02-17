window.onload = () => {
    try {
        unsubscribe();
    }
    catch(error) {
        console.error(error);
    }
}

async function search_and_display() {
    const search_input_el = document.getElementById("search_input");
    const search_results_el = document.getElementById("search_results");
    const search_timeout = 3;
    const update_timeout = 5;

    await window.__TAURI__.invoke("get_events_of", {
        filters: [{
            search: search_input_el.value,
            kinds: [1],
            limit: 10
        }],
        timeout: search_timeout
    })
    .then(async (results) => {
        results = JSON.parse(results);
        
        search_results_el.innerHTML = get_notes_html(results);

        await window.__TAURI__.invoke("req_events_of", {
            filters: [{
                authors: results.map((e) => e.pubkey),
                kinds: [0],
                limit: 5000
            }]
        });
    });

    while (true) {
        await window.__TAURI__.invoke("get_metadata")
            .then((metadata) => {
                metadata = JSON.parse(metadata);
                display_metadata(metadata);
            });

        await new Promise((resolve) => setTimeout(resolve, 1000 * update_timeout));
    }
}