let BaseDirectory, createDir, writeTextFile, readTextFile;

window.onload = () => {
    BaseDirectory = window.__TAURI__.fs.BaseDirectory;
    createDir = window.__TAURI__.fs.createDir;
    writeTextFile = window.__TAURI__.fs.writeTextFile;
    readTextFile = window.__TAURI__.fs.readTextFile;
}

const createDataFolder = async () => {
    try {
        await createDir("data", {
          dir: BaseDirectory.AppData,
          recursive: true,
        });
    } catch (error) {
        throw error;
    }
};

const createDataFile = async () => {
    try {
      await writeTextFile("data.json", "[]",
        {
            dir: BaseDirectory.AppData,
        }
      );
    } catch (error) {
        throw error;
    }
};

const readDataFile = async () => {
    try {
        return await readTextFile("data.json", {
            dir: BaseDirectory.AppData
        })
    } catch (error) {
        throw error;
    }
}
