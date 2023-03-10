const invoke = window.__TAURI__.invoke;
const clipboard = window.__TAURI__.clipboard;

window.onload = async function () {
    const public_key_el = document.getElementById("public_key");
    const secret_key_el = document.getElementById("secret_key");

    try {
        public_key_el.value = await invoke("get_public_key");
        secret_key_el.value = await invoke("get_secret_key");
    } catch (err) {
        console.error(err);
    }
};

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
