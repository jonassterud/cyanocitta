function set_pk_to_self() {
    window.__TAURI__.invoke("get_pk")
    .then((pk) => {
        window.localStorage.setItem("pk", pk);
    })
    .catch((error) => {
        console.error(error);
    });
}