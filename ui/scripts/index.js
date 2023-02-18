window.onload = () => {
    try {
        //unsubscribe();
        load_and_display_home(5);
    }
    catch(error) {
        console.error(error);
    }
}

/**
 * Starts a loop that receives notes and then displays them.
 * 
 * @param {Number} timeout - the time to wait between updates
 */
async function load_and_display_home(timeout) {
    const notes_el = document.getElementById("notes");
    
    let amount = 10;
    window.addEventListener("scroll", () => {
        if ((window.innerHeight + window.scrollY) >= document.body.offsetHeight) {
            amount += 10;
        }
    });

    while (true) {
        await window.__TAURI__.invoke("get_received_notes", {
            sort_by_date: true,
            amount: amount,
        })
        .then((notes) => {
            notes = JSON.parse(notes);
            notes_el.innerHTML = get_notes_html(notes);
        });

        await window.__TAURI__.invoke("get_metadata")
            .then((metadata) => {
                metadata = JSON.parse(metadata);
                display_metadata(metadata);
            });

        await new Promise((resolve) => setTimeout(resolve, 1000 * timeout));
    }
}