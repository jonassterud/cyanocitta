use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use std::time::Duration;
use tauri::State;

#[tauri::command]
pub async fn get_events_of(
    filters: Vec<SubscriptionFilter>,
    timeout: u64,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    let events = client
        .get_events_of(filters, Some(Duration::from_secs(timeout)))
        .await
        .map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&events).map_err(|e| e.to_string())?;

    Ok(json)
}
