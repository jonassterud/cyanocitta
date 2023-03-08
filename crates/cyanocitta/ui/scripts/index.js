function enter_secret_key() {
    const create_account_el = document.getElementById("create_account");
    const enter_secret_key_el = document.getElementById("enter_secret_key");
    const secret_key_el = document.getElementById("secret_key");

    create_account_el.value = "Import account";
    enter_secret_key_el.setAttribute("hidden", true);
    secret_key_el.removeAttribute("hidden");
}

function create_account() {
    const display_name = document.getElementById("display_name").value || null;
    const name = document.getElementById("name").value || null;
    const picture = document.getElementById("picture").value || null;
    const secret_key = document.getElementById("secret_key").value || null;

    console.log(display_name, name, picture, secret_key);
}
