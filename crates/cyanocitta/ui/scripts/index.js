const invoke = window.__TAURI__.invoke;

window.onload = function () {
    try {
        redirect_if_from_save();
    } catch (err) {
        console.error(err);
    }
};

/**
 * Redirect to home if state is from save.
 */
async function redirect_if_from_save() {
    const is_from_save = await invoke("is_from_save");

    if (is_from_save) {
        window.location.replace("pages/home.html");
    }
}

/**
 * Changes visiblity of secret key input element.
 */
function enter_secret_key() {
    const create_account_el = document.getElementById("create_account");
    const enter_secret_key_el = document.getElementById("enter_secret_key");
    const secret_key_el = document.getElementById("secret_key");

    create_account_el.value = "Import account";
    enter_secret_key_el.setAttribute("hidden", true);
    secret_key_el.removeAttribute("hidden");
}

/**
 * Set secret key, metadata, relays and then redirect to "home".
 *
 * @returns nothing.
 */
async function create_account() {
    const name = document.getElementById("name").value || null;
    const about = document.getElementById("about").value || null;
    const picture = document.getElementById("picture").value || null;
    const secret_key = document.getElementById("secret_key").value || null;
    const secret_key_is_hidden = document.getElementById("secret_key")?.hidden === true;

    try {
        // Set secret key
        if (secret_key_is_hidden === false) {
            if (secret_key === null) {
                let should_generate_secret_key = await custom_confirm(
                    "No secret key was specified, do you want to create a random one?",
                    "Yes - Create key",
                    "No - Go back"
                );

                if (should_generate_secret_key === false) {
                    return;
                }
            } else {
                await invoke("set_secret_key", { sk: secret_key });
            }
        }

        // Set metadata
        await invoke("set_metadata", {
            metadata: {
                name: name,
                about: about,
                picture: picture,
            },
        });

        // Add relays
        const relays = [...document.getElementsByClassName("relay selected")].map((relay_el) => relay_el.value);
        for (let relay_url of relays) {
            await invoke("add_relay", { url: relay_url });
        }

        // Redirect to home
        window.location.replace("pages/home.html");
    } catch (err) {
        console.error(err);
    }
}
