use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::Manager;

#[tauri::command]
pub async fn get_events_of(filters: Vec<SubscriptionFilter>, handle: tauri::AppHandle) -> Result<String, String> {
    let state = handle.state::<ClientState>();
    let inner = state.inner().0.lock().await;
    let client = &inner.client;
    let client = client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    let events = client.get_events_of(filters, None).await.map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&events).map_err(|e| e.to_string())?;

    Ok(json)
}