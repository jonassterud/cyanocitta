mod client_state;
mod commands;

use client_state::ClientState;
use anyhow::Result;
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

    pub async fn run(self) -> Result<()> {
        let mut client_state = ClientState::load().or_else(|_| ClientState::new())?;
        client_state.initialize_client().await?;
        client_state.start_notifications_loop().await?;

        let setup = self.setup;
        tauri::Builder::default()
            .manage(client_state)
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
            .run(tauri::generate_context!())?;

        Ok(())
    }
}
