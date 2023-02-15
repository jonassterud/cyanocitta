function set_viewing_pk_to_my_pk() {
    window.__TAURI__.invoke("get_my_pk")
        .then((my_pk) => {
            window.localStorage.setItem("viewing_pk", my_pk);
        })
        .catch((error) => {
            console.error(error);
        });
}

/*
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
*/