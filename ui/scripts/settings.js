window.onload = () => {
    try {
        load_and_display_relays();
        load_and_display_pk();
    }
    catch(error) {
        console.error(error);
    }
}

/**
 * Gets information about relays and then displays them.
 */
async function load_and_display_relays() {
    await window.__TAURI__.invoke("get_relays")
        .then((relays) => {
            let relays_container = document.getElementById("relays_container");
            relays_container.innerHTML = "";

            relays = JSON.parse(relays);
            relays.forEach(([relay_url, relay_status]) => {
                const connected_or_attempting = relay_status == "Connected" || relay_status == "Connecting" || relay_status == "Disconnected";
                relays_container.innerHTML += `
                    <div class="relay">
                        <div>
                            <input type="checkbox" ${connected_or_attempting ? "checked" : ""} onclick="change_relay_state(this, '${relay_url}')"></input>
                            <span>${relay_url}</span>
                        </div>
                        <span>${relay_status}</span>
                    </div>
                `;
            });
        });
}

/**
 * Adds a relay to the client.
 */
async function add_relay() {
    const add_relay_url_el = document.getElementById("add_relay_url");

    await window.__TAURI__.invoke("add_relay", { url: add_relay_url_el.value })
    await display_relays();
    add_relay_url_el.value = "";
}

/**
 * Connects/terminates to the relay depending on the value of the checkbox.
 * 
 * @param {HTMLInputElement} checkbox - checkbox to specify whether to connect/terminate
 * @param {String} relay_url - relay url
 */
async function change_relay_state(checkbox, relay_url) {
    if (checkbox.checked) {
        await window.__TAURI__.invoke("connect_relay", { url: relay_url });
    } else {
        await window.__TAURI__.invoke("disconnect_relay", { url: relay_url });
    }

    await display_relays();
}

/**
 * Gets the public key of this client and displays it.
 */
async function load_and_display_pk() {
    const pk = await window.__TAURI__.invoke("get_my_pk");

    document.getElementById("pk").value = pk;
}

/**
 * Creates a new client from the value of the secret key input element.
 */
async function set_new_sk() {
    if (confirm("This will generate Nostr keys from the given secret key, and will not save any previous keys.\n\nAre you sure you wish to proceed?")) {
        const sk_el = document.getElementById("sk");

        await window.__TAURI__.invoke("set_new_sk", { sk: sk_el.value });
        sk_el.value = "";

        await display_pk();
    }
}