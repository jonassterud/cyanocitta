var invoke, BaseDirectory, createDir, writeTextFile, readTextFile, exists, removeFile; // Tauri
var profiles_el, secret_key_el, create_profile_el; // HTML elements
var profiles = []; // Global

window.onload = async () => {
    // Load Tauri APIs
    invoke = window.__TAURI__.invoke;
    BaseDirectory = window.__TAURI__.fs.BaseDirectory;
    createDir = window.__TAURI__.fs.createDir;
    writeTextFile = window.__TAURI__.fs.writeTextFile;
    readTextFile = window.__TAURI__.fs.readTextFile;
    exists = window.__TAURI__.fs.exists;
    removeFile = window.__TAURI__.fs.removeFile;

    // Load HTML elements
    profiles_el = document.getElementById("profiles");
    secret_key_el = document.getElementById("secret-key");
    create_profile_el = document.getElementById("create-profile");

    load().catch((error) => {
        console.error(error);
    });
}

async function load() {
    // Load existing profiles
    if (await dataFileExists()) {
        let data = await readDataFile();
        profiles = JSON.parse(data);
    }

    let display_profiles = () => {
        profiles_el.innerHTML = "";
        profiles.forEach((profile) => {
            profiles_el.innerHTML += `<div>
                <span>Secret: <code>${profile.secret_key}</code></span>
                <br>
                <span>Public: <code>${profile.public_key}</code></span>
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
        profiles.push(JSON.parse(new_profile));
        await createDataFile(JSON.stringify(profiles));
        display_profiles();
    });
}

async function createDataFolder() {
    return createDir("data", {
        dir: BaseDirectory.AppData,
        recursive: true,
    });  
};

async function createDataFile(json) {
    return writeTextFile("data.json", json, {
        dir: BaseDirectory.AppData,
    });
};

async function readDataFile() {
    return readTextFile("data.json", {
        dir: BaseDirectory.AppData
    });
}

async function dataFileExists() {
    return exists("data.json", {
        dir: BaseDirectory.AppData
    });
}

async function removeDataFile() {
    return removeFile("data.json", {
        dir: BaseDirectory.AppData
    });
}