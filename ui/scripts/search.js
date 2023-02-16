async function search() {
    const search_input_el = document.getElementById("search_input");
    const search_results_el = document.getElementById("search_results");
    const timeout = 3;

    await window.__TAURI__.invoke("get_events_of", {
        filters: [{
            search: search_input_el.value,
            kinds: [1],
            limit: 10
        }],
        timeout: timeout
    })
    .then((results) => {
        results = JSON.parse(results);
        search_results_el.innerHTML = get_notes_html(results);
    });

    // then start a subscription for metadata..
    // and update metadata in loop.

    await window.__TAURI__.invoke("get_metadata")
    .then((metadata) => {
        metadata = JSON.parse(metadata);
        display_metadata(metadata);
    });
}