mod commands;
mod nostr_tools;

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
        use nostr_tools::Client;
        let client = Client::load().unwrap_or_else(|_| Client::new_random());
        Client::start(client.clone());

        let setup = self.setup;
        tauri::Builder::default()
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .manage(client)
            .invoke_handler(tauri::generate_handler![
                commands::subscribe,
                commands::unsubscribe,
                commands::notes,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
