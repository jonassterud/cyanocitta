function _get_events_of(pk, kinds, timeout) {
    return window.__TAURI__.invoke("get_events_of", {
        filters: [
            {
                authors: [pk],
                kinds: kinds,
                limit: 5000,
            }
        ],
        timeout: timeout,
    })
        .then((response) => {
            return JSON.parse(response);
        });
}

async function load_user_notes(pk) {
    let events = await _get_events_of(pk, [0, 1, 2], 10);
    let metadata = null;

    console.log(events);
    events.forEach((event) => {
        switch (event.kind) {
            case 0:
                if (metadata === null) {
                    metadata = JSON.parse(event.content);

                    document.getElementById("name").innerHTML = metadata.name;
                    document.getElementById("display_name").innerHTML = metadata.display_name;
                    document.getElementById("about").innerHTML = metadata.about;
                    document.getElementById("picture").src = metadata.picture;
                }

                break;
            case 1:
                console.log(event);
                break;
            case 2:
                console.log(`recommended relay: ${event.content}`);
                break;
        }
    });
   
}