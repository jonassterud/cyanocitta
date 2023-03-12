const invoke = window.__TAURI__.invoke;
const clipboard = window.__TAURI__.clipboard;

window.onload = async function () {
    const public_key_el = document.getElementById("public_key");
    const secret_key_el = document.getElementById("secret_key");
    const relays_el = document.getElementById("relays");

    try {
        public_key_el.value = await invoke("get_public_key");
        secret_key_el.value = await invoke("get_secret_key");

        const relays = await invoke("get_relays");
        for (const [relay_url, relay_is_active] of relays) {
            const relay_el = get_relay_element(relay_url);
            if (relay_is_active) relay_el.classList.add("selected");

            relays_el.prepend(relay_el);
        }
    } catch (err) {
        console.error(err);
    }
};

/**
 * Add relay input element.
 *
 * @param {String} relay_url - "wss://" relay url.
 * @returns {HTMLInputElement} relay element.
 */
function get_relay_element(relay_url) {
    const relay_el = document.createElement("input");
    relay_el.classList.add("button", "relay");
    relay_el.type = "button";
    relay_el.value = relay_url;
    relay_el.addEventListener("click", () => {
        const is_activating = relay_el.classList.contains("selected") === false;

        if (is_activating) {
            invoke("add_relay", { url: relay_url, buffer: 100 })
                .then(() => {
                    toggle_class(relay_el, "selected");
                })
                .catch((err) => {
                    console.error(err);
                });
        } else {
            invoke("remove_relay", { url: relay_url })
                .then(() => {
                    toggle_class(relay_el, "selected");
                })
                .catch((err) => {
                    console.error(err);
                });
        }
    });

    return relay_el;
}

function add_relay() {
    const relays_el = document.getElementById("relays");
    const input_relay_el = document.getElementById("input_relay");
    const relay_el = get_relay_element(input_relay_el.value);

    input_relay_el.value = "";
    relays_el.prepend(relay_el);
}

/**
 * Copies an HTML input's value to the clipboard.
 *
 * @param {String} id - HTML input element id.
 * @param {String} message - alert message to display.
 * @param {Number} timeout - seconds to display message for.
 */
async function copy_input_value(id, message = null, timeout = 1.2) {
    const el = document.getElementById(id);
    clipboard.writeText(el.value).catch((err) => {
        console.error(err);
    });

    if (message !== null) {
        custom_alert(message, timeout);
    }
}
