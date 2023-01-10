mod commands;

use anyhow::Result;
use cyanocitta_nostr_tools::{Client, AppData};
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

        let client = Client::load().unwrap_or(Client::new_default_relays());
        let app_data = client.app_data.clone();

        tauri::Builder::default()
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .manage(app_data)
            .invoke_handler(tauri::generate_handler![
                commands::new_profile,
                commands::get_app_data,
                commands::set_current_profile,
                commands::start_subscription,
                commands::stop_subscription,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
