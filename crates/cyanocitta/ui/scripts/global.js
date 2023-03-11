/**
 * Fancy styled version of the Javascript confirm function.
 *
 * @param {String} message - text to display in the confirm box.
 * @param {String} true_text - text to show for the input option returning `true`.
 * @param {String} false_text - text to show for the input option returning `false`.
 * @returns {Promise<bool>} user result.
 */
async function custom_confirm(message, true_text = "Yes", false_text = "No") {
    const custom_confirm_container_el = document.createElement("div");
    custom_confirm_container_el.setAttribute("id", "custom_confirm_container");

    custom_confirm_container_el.innerHTML += `
    <div id="custom_confirm">
        <p>${message}</p>
        <div id="custom_confirm_input_container">
            <input class="button" type="button" id="custom_confirm_true" value="${true_text}" />
            <input class="button" type="button" id="custom_confirm_false" value="${false_text}" />
        </div>
    </div>`;

    document.body.style = "overflow: hidden";
    document.body.appendChild(custom_confirm_container_el);

    try {
        return await new Promise((resolve) => {
            document.getElementById("custom_confirm_true").addEventListener("click", () => {
                resolve(true);
            });
            document.getElementById("custom_confirm_false").addEventListener("click", () => {
                resolve(false);
            });
        });
    } finally {
        document.body.style = "";
        custom_confirm_container_el.remove();
    }
}

/**
 * Fancy styled version of the Javascript alert function.
 *
 * @param {String} message - text to display in the alert box.
 * @param {Number} timeout - seconds to display message for.
 */
function custom_alert(message, timeout) {
    const custom_alert_container_el = document.createElement("div");
    custom_alert_container_el.setAttribute("id", "custom_alert_container");

    custom_alert_container_el.innerHTML += `
    <div id="custom_alert">
        <p>${message}</p>    
    </div>`;

    document.body.appendChild(custom_alert_container_el);

    setTimeout(() => {
        custom_alert_container_el.classList.add("remove");
        setTimeout(() => {
            custom_alert_container_el.remove();
        }, 2000);
    }, timeout * 1000);
}

/**
 * Toggle class on element.
 *
 * @param {HTMLElement} el - element to toggle class on.
 * @param {String} name - name of class.
 */
function toggle_class(el, name) {
    const is_selected = el.classList.contains(name);

    if (is_selected) el.classList.remove(name);
    else el.classList.add(name);
}
