window.onload = () => {
    try {
        load_user_notes("84b73204d850c7eadc2a3ff96728cb461a35951216475f08fae970b90eb55ee4");
    }
    catch(error) {
        console.error(error);
    }
}

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

                    [...document.getElementsByClassName("name")].forEach((e) => e.innerHTML = metadata.name);
                    [...document.getElementsByClassName("display_name")].forEach((e) => e.innerHTML = metadata.display_name);
                    [...document.getElementsByClassName("about")].forEach((e) => e.innerHTML = metadata.about);
                    [...document.getElementsByClassName("picture")].forEach((e) => e.src = metadata.picture);
                }

                break;
            case 1:
                document.getElementById("notes").innerHTML += `<span>${event.content}</span><br>`;
                console.log(event);
                break;
            case 2:
                console.log(`recommended relay: ${event.content}`);
                break;
        }
    });
   
}