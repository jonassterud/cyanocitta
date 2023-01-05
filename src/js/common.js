var app_data = {
    profiles: [],
    current_profile: undefined
};

async function load_app_data() {
    if (await _dataFileExists()) {
        let data = await _readDataFile();
        app_data = JSON.parse(data);
    }
}

async function save_app_data() {
    await _createDataFile(JSON.stringify(app_data));
}

async function _createDataFolder() {
    const createDir = window.__TAURI__.fs.createDir;
    const BaseDirectory = window.__TAURI__.fs.BaseDirectory;

    return createDir("data", {
        dir: BaseDirectory.AppData,
        recursive: true,
    });  
};

async function _createDataFile(json) {
    const writeTextFile = window.__TAURI__.fs.writeTextFile;
    const BaseDirectory = window.__TAURI__.fs.BaseDirectory;

    return writeTextFile("data.json", json, {
        dir: BaseDirectory.AppData,
    });
};

async function _readDataFile() {
    const readTextFile = window.__TAURI__.fs.readTextFile;
    const BaseDirectory = window.__TAURI__.fs.BaseDirectory;

    return readTextFile("data.json", {
        dir: BaseDirectory.AppData
    });
}

async function _dataFileExists() {
    const exists = window.__TAURI__.fs.exists;
    const BaseDirectory = window.__TAURI__.fs.BaseDirectory;

    return exists("data.json", {
        dir: BaseDirectory.AppData
    });
}

async function _removeDataFile() {
    const removeFile = window.__TAURI__.fs.removeFile;
    const BaseDirectory = window.__TAURI__.fs.BaseDirectory;

    return removeFile("data.json", {
        dir: BaseDirectory.AppData
    });
}