use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn get_metadata(state: State<'_, ClientState>) -> Result<String, String> {
    let inner = state.0.lock().await;
    let metadata = &inner.metadata;
    let json = serde_json::to_string(metadata).map_err(|e| e.to_string())?;

    Ok(json)
}

#[tauri::command]
pub async fn set_metadata(metadata: String, state: State<'_, ClientState>) -> Result<(), String> {
    let metadata = serde_json::from_str::<Metadata>(&metadata).map_err(|e| e.to_string())?;
    let mut inner = state.0.lock().await;
    inner.metadata = metadata.clone();

    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client
        .set_metadata(metadata)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
