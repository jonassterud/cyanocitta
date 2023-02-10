use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::Manager;

#[tauri::command]
pub async fn get_metadata(handle: tauri::AppHandle) -> Result<String, String> {
    let state = handle.state::<ClientState>();
    let inner = state.inner().0.lock().await;
    let metadata = &inner.metadata;
    let json = serde_json::to_string(metadata).map_err(|e| e.to_string())?;

    Ok(json)
}

#[tauri::command]
pub async fn set_metadata(metadata: String, handle: tauri::AppHandle) -> Result<(), String> {
    let metadata = serde_json::from_str::<Metadata>(&metadata).map_err(|e| e.to_string())?;
    let state = handle.state::<ClientState>();
    let mut inner = state.inner().0.lock().await;
    inner.metadata = metadata.clone();

    let client = &inner.client;
    let client = client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;
    client
        .set_metadata(metadata)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
