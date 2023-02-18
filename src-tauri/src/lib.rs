mod client_state;
mod commands;
mod notifications;

use anyhow::Result;
use client_state::*;
use nostr_sdk::prelude::*;
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
        let client_state = ClientState::load().or_else(|_| ClientState::new(&Keys::generate()))?;
        client_state.initialize_client().await?;

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
                commands::set_metadata,
                commands::get_events_of,
                commands::publish_text_note,
                commands::get_my_pk,
                commands::save_and_exit,
                commands::get_received_notes,
                commands::req_events_of,
                commands::subscribe,
                commands::get_relays,
                commands::add_relay,
                commands::disconnect_relay,
                commands::connect_relay,
                commands::set_new_sk,
                commands::unsubscribe_and_reset,
                commands::follow,
                commands::unfollow,
                commands::get_following,
                commands::is_following,
            ])
            .run(tauri::generate_context!())?;

        Ok(())
    }
}
