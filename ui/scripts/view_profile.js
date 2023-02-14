window.onload = () => {
    try {
        load_profile(5); // timeout???
    }
    catch(error) {
        console.error(error);
    }
}

function load_profile(timeout) {
    let pk = window.localStorage.getItem("pk");
    let metadata = null;
    let notes = new Map();

    if (pk === null) {
        throw Error("missing public key");
    }

    window.__TAURI__.invoke("get_events_of", {
        filters: [{ authors: [pk], kinds: [0, 1, 2], limit: 5000 }],
        timeout: timeout,
    })
    .then((events) => JSON.parse(events))
    .then((events) => {
        events.forEach((event) => {
            switch (event.kind) {
                case 0:
                    if (metadata === null) {
                        metadata = JSON.parse(event.content);
                    }
    
                    break;
                case 1:
                    notes.set(event.id, event);

                    break;
                case 2:
                    console.log(`recommended relay: ${event.content}`);

                    break;
            }
        });
    })
    .then(() => {
        notes.forEach((note) => {
            document.getElementById("notes").innerHTML += `
                <div class="note">
                    <img class="picture" id="note_picture" src="media/avatar-default.svg">
                    <div>
                        <div>
                            <span class="display_name" id="note_display_name">Display name</span>
                            <span class="name" id="note_name">@Username</span>
                        </div>
                        <span class="note_content">${note.content}</span>
                    </div>
                </div>
            `;
        });
    })
    .then(() => {
        [...document.getElementsByClassName("name")].forEach((e) => e.innerHTML = metadata?.name || pk);
        [...document.getElementsByClassName("display_name")].forEach((e) => e.innerHTML = metadata?.display_name || pk.substring(0, 8) + "...");
        [...document.getElementsByClassName("about")].forEach((e) => e.innerHTML = metadata?.about || "");
        [...document.getElementsByClassName("picture")].forEach((e) => e.src = metadata?.picture || "media/avatar-default.svg");
    })
    .catch((error) => {
        throw error;
    });
}