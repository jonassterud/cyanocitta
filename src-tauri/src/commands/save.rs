use crate::client_state::ClientState;
use nostr_sdk::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn save_state(state: State<'_, ClientState>) -> Result<(), String> {
    let inner = state.0.lock().await;
    inner.save().map_err(|e| e.to_string())?;

    Ok(())
}
