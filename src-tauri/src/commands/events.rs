use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use std::time::Duration;
use tauri::State;

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
#[tauri::command]
pub async fn req_events_of(
    filters: Vec<Filter>,
    timeout: Option<u64>,
    state: State<'_, ClientState>,
) -> Result<(), String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client
        .req_events_of(filters, timeout.map(Duration::from_secs))
        .await;

    Ok(())
}

/// Get recieved notes.
///
/// # Errors
///
/// This function will return an error if:
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn get_received_notes(
    pk: Option<String>,
    sort_by_date: Option<bool>,
    amount: usize,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let inner = state.0.lock().await;
    let mut notes: Vec<&Event> = if let Some(pk) = pk {
        inner
            .notes
            .iter()
            .map(|e| e.1)
            .filter(|e| e.pubkey.to_string() == pk)
            .collect()
    } else {
        inner.notes.iter().map(|e| e.1).collect()
    };

    if let Some(true) = sort_by_date {
        notes.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    }

    notes.truncate(amount);
    let json = serde_json::to_string(&notes).map_err(|e| e.to_string())?;

    Ok(json)
}
