mod client_state;
mod commands;

use client_state::ClientState;
use std::sync::Arc;
use tauri::App;
use tokio::sync::Mutex;

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

    pub async fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .manage(Arc::new(Mutex::new(
                ClientState::load()
                    .or(ClientState::new().await)
                    .expect("failed getting client state"),
            )))
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                commands::get_metadata,
                commands::set_metadata
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
