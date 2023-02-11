use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::Manager;

#[tauri::command]
pub async fn publish_text_note(content: String, handle: tauri::AppHandle) -> Result<(), String> {
    let state = handle.state::<ClientState>();
    let inner = state.inner().0.lock().await;
    let client = &inner.client;
    let client = client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client.publish_text_note(content, &[]).await.map_err(|e| e.to_string())?;

    Ok(())
}