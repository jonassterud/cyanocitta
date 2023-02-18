use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::State;

/// Get stored metadata for `pk` or return all metadata.
///
/// # Errors
///
/// This function will return an error if:
/// * No metadata was found for `pk`.
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn get_metadata(state: State<'_, ClientState>,) -> Result<String, String> {
    let metadata = &mut state.0.lock().await.metadata;
    let json = serde_json::to_string(metadata).map_err(|e| e.to_string())?;

    Ok(json)
}

/// Update metadata.
///
/// # Errors
///
/// This function will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
/// * [`Client::set_metadata`] fails.
#[tauri::command]
pub async fn set_metadata(metadata: Metadata, state: State<'_, ClientState>) -> Result<(), String> {
    let inner = state.0.lock().await;
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
