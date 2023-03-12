mod app_state;
mod commands;

use anyhow::Result;
use app_state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_public_key,
            commands::get_secret_key,
            commands::set_secret_key,
            commands::is_from_save,
            commands::set_metadata,
            commands::add_relay,
            commands::remove_relay,
            commands::get_relays,
            commands::_save_state,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
