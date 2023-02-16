use crate::client_state::ClientState;
use nostr_sdk::prelude::*;
use tauri::State;
use anyhow::anyhow;

#[tauri::command]
pub async fn exit_and_save(state: State<'_, ClientState>) -> Result<(), String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .to_owned()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    inner.save().map_err(|e| e.to_string())?;
    client.unsubscribe().await;
    client.disconnect().await.map_err(|e| e.to_string())?;
    client.shutdown().await.map_err(|e| e.to_string())?;

    Ok(())
}
