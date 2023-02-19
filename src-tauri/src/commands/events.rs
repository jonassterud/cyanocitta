use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use std::time::Duration;
use tauri::State;
/*
/// Get events of filters.
///
/// # Errors
///
/// This function will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
/// * [`Client::get_events_of`] fails.
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn get_events_of(
    filters: Vec<Filter>,
    timeout: Option<u64>,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    let events = client
        .get_events_of(filters, timeout.map(Duration::from_secs))
        .await
        .map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&events).map_err(|e| e.to_string())?;

    Ok(json)
}

/// Request events of filters.
///
/// # Errors
///
/// This function will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
/// * [`Client::req_events_of`] fails.
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn req_events_of(
    filters: Vec<Filter>,
    timeout: Option<u64>,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    let subscription_ids = client
        .req_events_of(filters, timeout.map(Duration::from_secs))
        .await;
    let json = serde_json::to_string(&subscription_ids).map_err(|e| e.to_string())?;

    Ok(json)
}
*/

/// Get recieved notes.
///
/// # Errors
///
/// This function will return an error if:
/// * No notes were found.
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn get_received_notes(
    subscription_id: String,
    amount: Option<usize>,
    sort: Option<bool>,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let inner = state.0.lock().await;
    let mut notes = inner
        .notes
        .get(&subscription_id)
        .ok_or_else(|| anyhow!("no notes found").to_string())?
        .iter()
        .map(|(_, event)| event)
        .collect::<Vec<&Event>>();
    if let Some(true) = sort {
        notes.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    }
    if let Some(amount) = amount {
        notes.truncate(amount);
    }

    let json = serde_json::to_string(&notes).map_err(|e| e.to_string())?;

    Ok(json)
}
