window.onload = () => {
    try {
        display_relays();
    }
    catch(error) {
        console.error(error);
    }
}

async function display_relays() {
    await window.__TAURI__.invoke("get_relays")
        .then((relays) => {
            let container = document.getElementById("relays");
            container.innerHTML = "";

            relays = JSON.parse(relays);
            relays.forEach(([url, status]) => {
                const connected_or_attempting = status == "Connected" || status == "Connecting";

                container.innerHTML += `
                    <div class="relay">
                        <div>
                            <input type="checkbox" ${connected_or_attempting ? "checked" : ""} onclick="change_relay_state(this, '${url}')"></input>
                            <span class="url">${url}</span>
                        </div>
                        <span class="status">${status}</span>
                    </div>
                `;
            });
        })
        .catch((error) => {
            throw error;
        });
}

async function add_relay() {
    let url_element = document.getElementById("url");

    await window.__TAURI__.invoke("add_relay", { url: url_element.value })
        .then(async () => {
            url_element.innerHTML = "";
            await display_relays();
        })
        .catch((error) => {
            console.error(error);
        })
}

async function change_relay_state(checkbox, url) {
    if (checkbox.checked) {
        await window.__TAURI__.invoke("connect_relay", { url: url })
            .then(async () => {
                await display_relays();
            })
            .catch((error) => {
                console.error(error);
            });
    } else {
        await window.__TAURI__.invoke("disconnect_relay", { url: url })
            .then(async () => {
                await display_relays();
            })
            .catch((error) => {
                console.error(error);
            });
    }
}