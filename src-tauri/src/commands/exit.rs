use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn save_and_exit(state: State<'_, ClientState>) -> Result<(), String> {
    let inner = state.0.lock().await;
    inner.save().map_err(|e| e.to_string())?;

    // requires internet connection:
    /*
    let client = inner
        .client
        .to_owned()
        .ok_or_else(|| anyhow!("missing client").to_string())?;
    client.unsubscribe().await;
    client.disconnect().await.map_err(|e| e.to_string())?;
    client.shutdown().await.map_err(|e| e.to_string())?; // uses timeout
    */
    Ok(())
}
