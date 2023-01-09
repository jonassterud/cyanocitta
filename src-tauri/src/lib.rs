mod commands;

use cyanocitta_nostr_tools::AppData;
use std::sync::Mutex;
use tauri::App;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .manage(Mutex::new(
                AppData::load().unwrap_or(AppData::new_default_relays()),
            ))
            .invoke_handler(tauri::generate_handler![
                commands::new_profile,
                commands::get_app_data,
                commands::set_current_profile,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
