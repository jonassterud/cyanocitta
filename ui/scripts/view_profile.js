function _get_events_of() {
    return window.__TAURI__.invoke("get_events_of", {
        filters: [
            {
                authors: ["84b73204d850c7eadc2a3ff96728cb461a35951216475f08fae970b90eb55ee4"],
                limit: 2,
            }
        ]
    })
        .then((response) => {
            return JSON.parse(response);
        });
}

async function load_user_notes() {
   
}