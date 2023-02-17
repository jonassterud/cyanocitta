use crate::client_state::{ClientState, InnerClientState};
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::State;

/// Subscribe to filters.
///
/// # Errors
///
/// This function will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
#[tauri::command]
pub async fn subscribe(
    filters: Vec<Filter>,
    state: State<'_, ClientState>,
) -> Result<(), String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client.subscribe(filters).await;

    Ok(())
}
