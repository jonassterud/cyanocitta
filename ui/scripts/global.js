function get_metadata_from_pk(pk, timeout) {
    return window.__TAURI__.invoke("get_events_of", {
        filters: [{ authors: [pk], kinds: [0], limit: 1 }],
        timeout: timeout
    })
    .then((events) => JSON.parse(events)[0])
    .then((event) => {
        if (event === undefined) {
            throw Error("no metadata found");
        }

        return JSON.parse(event.content);
    });
}