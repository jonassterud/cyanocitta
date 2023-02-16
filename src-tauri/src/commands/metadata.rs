use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use std::collections::BTreeMap;
use tauri::State;

/// Get stored metadata for `pk` or return all metadata.
///
/// # Errors
///
/// This function will return an error if:
/// * No metadata was found for `pk`.
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn get_metadata(
    pk: Option<String>,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let metadata = &mut state.0.lock().await.metadata;

    if let Some(pk) = pk {
        let specific_metadata = metadata
            .get(&pk)
            .ok_or_else(|| anyhow!("no metadata found").to_string())?;
        let mut map = BTreeMap::new();
        map.insert(pk, specific_metadata);

        serde_json::to_string(&map).map_err(|e| e.to_string())
    } else {
        serde_json::to_string(metadata).map_err(|e| e.to_string())
    }
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
