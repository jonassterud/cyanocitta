async function custom_prompt(message, true_text = "Yes", false_text = "No") {
    document.body.innerHTML += `
        <div id="custom_prompt">
            <div>
                <p>${message}</p>
                <div>
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
        document.getElementById("custom_prompt").remove();
    }
}
