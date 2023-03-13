const invoke = window.__TAURI__.invoke;
const clipboard = window.__TAURI__.clipboard;

window.onload = function () {
    try {
        load_keys();
        load_relays();
        load_metadata();
    } catch (err) {
        console.error(err);
    }
};

/**
 * Loads public- and secret key and displays them.
 */
async function load_keys() {
    const public_key_el = document.getElementById("public_key");
    const secret_key_el = document.getElementById("secret_key");

    public_key_el.value = await invoke("get_public_key");
    secret_key_el.value = await invoke("get_secret_key");
}

/**
 * Loads relays, creates HTML elements, and displays them.
 */
async function load_relays() {
    const relays_el = document.getElementById("relays");

    const relays = await invoke("get_relays");
    for (const [relay_url, relay_is_active] of relays) {
        const relay_el = get_relay_element(relay_url);
        if (relay_is_active) relay_el.classList.add("selected");

        relays_el.prepend(relay_el);
    }
}

/**
 * Loads metadata and displays it.
 */
async function load_metadata() {
    const name_el = document.getElementById("name");
    const about_el = document.getElementById("about");
    const picture_el = document.getElementById("picture");
    const metadata = await invoke("get_metadata");

    name_el.value = metadata.name || "";
    about_el.value = metadata.about || "";
    picture_el.value = metadata.picture || "";
}

/**
 * Update profile metadata.
 */
function apply_profile() {
    const name = document.getElementById("name").value;
    const about = document.getElementById("about").value;
    const picture = document.getElementById("picture").value;

    invoke("set_metadata", {
        metadata: {
            name: name,
            about: about,
            picture: picture,
        },
    })
        .then(() => {
            custom_alert("Updated profile", 1.2);
        })
        .catch((err) => {
            console.error(err);
        });
}

/**
 * Create relay HTML element.
 *
 * @param {String} relay_url - "wss://" relay url.
 * @returns {HTMLInputElement} relay element.
 */
function get_relay_element(relay_url) {
    const relay_el = document.createElement("input");
    relay_el.classList.add("button", "relay");
    relay_el.type = "button";
    relay_el.value = relay_url;
    relay_el.addEventListener("click", () => handle_relay_click(relay_el));

    return relay_el;
}

/**
 * Handle relay click.
 *
 * @param {HTMLInputElement} el - relay input element.
 */
async function handle_relay_click(el) {
    const is_activating = el.classList.contains("selected") === false;

    try {
        if (is_activating) {
            toggle_class(el, "loading");
            await invoke("add_relay", { url: el.value });
            await invoke("listen_relay", { url: el.value, buffer: 100 });
            toggle_class(el, "loading");
        } else {
            await invoke("remove_relay", { url: el.value });
        }

        toggle_class(el, "selected");
    } catch (err) {
        console.error(err);
    }
}

/**
 * Reads input value and adds relay HTML element.
 */
function add_relay_element() {
    const relays_el = document.getElementById("relays");
    const input_relay_el = document.getElementById("input_relay");
    const relay_el = get_relay_element(input_relay_el.value);

    input_relay_el.value = "";
    relays_el.prepend(relay_el);
    relay_el.click();
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
