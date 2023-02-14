use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn get_metadata(pk: String, state: State<'_, ClientState>) -> Result<String, String> {
    let inner = state.0.lock().await;
    let metadata = &inner.metadata.get(&pk);
    let json = serde_json::to_string(metadata).map_err(|e| e.to_string())?;

    Ok(json)
}

#[tauri::command]
pub async fn set_metadata(metadata: Metadata, state: State<'_, ClientState>) -> Result<(), String> {
    let inner = &mut *state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    inner
        .metadata
        .insert(inner.pk.to_string(), metadata.clone());
    client
        .set_metadata(metadata)
        .await
        .map_err(|e| e.to_string())?;
    inner.save().map_err(|e| e.to_string())?;

    Ok(())
}
