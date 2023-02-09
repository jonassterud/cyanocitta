use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_metadata(handle: tauri::AppHandle) -> Result<String, String> {
    let state = handle.state::<Arc<Mutex<ClientState>>>();
    let metadata = &state.lock().await.metadata;
    let json = serde_json::to_string(metadata).map_err(|e| e.to_string())?;

    Ok(json)
}

#[tauri::command]
pub async fn set_metadata(metadata: String, handle: tauri::AppHandle) -> Result<(), String> {
    let metadata = serde_json::from_str::<Metadata>(&metadata).map_err(|e| e.to_string())?;
    let state = handle.state::<Arc<Mutex<ClientState>>>();
    state.lock().await.metadata = metadata.clone();

    let client = &state.lock().await.client;
    let client = client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;
    client
        .set_metadata(metadata)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
