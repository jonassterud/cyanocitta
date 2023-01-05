var invoke; // Tauri
var profiles_el, secret_key_el, create_profile_el; // HTML elements

window.onload = async () => {
    // Load Tauri APIs
    invoke = window.__TAURI__.invoke;

    // Load HTML elements
    profiles_el = document.getElementById("profiles");
    secret_key_el = document.getElementById("secret-key");
    create_profile_el = document.getElementById("create-profile");

    load().catch((error) => {
        console.error(error);
    });
}

async function load() {
    // Load app data
    await load_app_data();

    // Display profiles
    let display_profiles = () => {
        profiles_el.innerHTML = "";
        app_data.profiles.forEach((profile, index) => {
            profiles_el.innerHTML += `<div>
                <span onclick="login_with_profile(${index})">
                    Secret: <code>${profile.secret_key}</code>
                </span>
                <br>
                <span>
                    Public: <code>${profile.public_key}</code>
                </span>
            </div>`;
        });
    };
    display_profiles();

    // Change button text based on input value
    secret_key_el.addEventListener("input", () => {
        if (secret_key_el.value.length !== 0) {
            create_profile_el.innerHTML = create_profile_el.innerHTML.replace("Create", "Add");
        } else {
            create_profile_el.innerHTML = create_profile_el.innerHTML.replace("Add", "Create");
        }
    });

    // Create new profile (on button click)
    create_profile_el.addEventListener("click", async () => {
        let sk = secret_key_el.value.length === 0 ? null : secret_key_el.value;
        let new_profile = await invoke("new_profile", { secret_key: sk });
        app_data.profiles.push(JSON.parse(new_profile));
        await save_app_data();
        display_profiles();
    });
}

function login_with_profile(profile_index) {
    app_data.current_profile = profile_index;
    save_app_data.then(() => {
        window.location = "/home.html";
    }).catch((error) => {
        console.error(error);
    })
}
