function set_viewing_pk_to_my_pk() {
    window.__TAURI__.invoke("get_my_pk")
        .then((my_pk) => {
            window.localStorage.setItem("viewing_pk", my_pk);
        })
        .catch((error) => {
            console.error(error);
        });
}