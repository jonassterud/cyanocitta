mod app_state;
mod commands;

use anyhow::Result;
use app_state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![commands::set_secret_key, commands::is_from_save])
        .run(tauri::generate_context!())?;

    Ok(())
}
