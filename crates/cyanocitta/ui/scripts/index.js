const invoke = window.__TAURI__.invoke;

window.onload = async function () {
    try {
        const is_from_save = await invoke("is_from_save");
        if (is_from_save) {
            window.location.replace("pages/home.html");
        }
    } catch (err) {
        console.error(err);
    }
};

function enter_secret_key() {
    const create_account_el = document.getElementById("create_account");
    const enter_secret_key_el = document.getElementById("enter_secret_key");
    const secret_key_el = document.getElementById("secret_key");

    create_account_el.value = "Import account";
    enter_secret_key_el.setAttribute("hidden", true);
    secret_key_el.removeAttribute("hidden");
}

async function create_account() {
    const display_name = document.getElementById("display_name").value || null;
    const name = document.getElementById("name").value || null;
    const picture = document.getElementById("picture").value || null;
    const secret_key = document.getElementById("secret_key").value || null;

    try {
        // Set metadata
        await invoke("set_metadata", {
            metadata: {
                name: name,
                picture: picture,
            },
        });

        // Add relays
        const relays = [...document.getElementsByClassName("relay selected")].map((relay_el) => relay_el.value);
        for (let relay_url of relays) {
            await invoke("add_relay", { url: relay_url, buffer: 100 }).catch((err) => {
                console.error(err);
            });
        }

        // Set secret key
        if (secret_key === null) {
            window.location.replace("pages/home.html");
        } else {
            await invoke("set_secret_key", { sk: secret_key });
            window.location.replace("pages/home.html");
        }
    } catch (err) {
        console.error(err);
    }
}

function select_relay(relay_el) {
    const is_selected = relay_el.classList.contains("selected");

    if (is_selected) relay_el.classList.remove("selected");
    else relay_el.classList.add("selected");
}
