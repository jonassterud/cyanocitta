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
