async function custom_prompt(message, true_text = "Yes", false_text = "No") {
    document.body.style = "overflow: hidden";
    document.body.innerHTML += `
        <div id="custom_prompt_container">
            <div id="custom_prompt">
                <p>${message}</p>
                <div id="custom_prompt_input_container">
                    <input class="button" type="button" id="custom_prompt_true" value="${true_text}" />
                    <input class="button" type="button" id="custom_prompt_false" value="${false_text}" />
                </div>
            </div>
        </div>
    `;

    try {
        return await new Promise((resolve) => {
            document.getElementById("custom_prompt_true").addEventListener("click", () => {
                resolve(true);
            });
            document.getElementById("custom_prompt_false").addEventListener("click", () => {
                resolve(false);
            });
        });
    } finally {
        document.body.style = "";
        document.getElementById("custom_prompt_container").remove();
    }
}
