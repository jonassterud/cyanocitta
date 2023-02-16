use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn get_relays(state: State<'_, ClientState>) -> Result<String, String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    let mut relays = vec![];
    for (url, relay) in client.relays().await {
        relays.push((url.to_string(), relay.status().await.to_string()));
    }
    let json = serde_json::to_string(&relays).map_err(|e| e.to_string())?;

    Ok(json)
}

#[tauri::command]
pub async fn add_relay(url: String, state: State<'_, ClientState>) -> Result<(), String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client
        .add_relay(url, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn disconnect_relay(url: String, state: State<'_, ClientState>) -> Result<(), String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client
        .disconnect_relay(url)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn connect_relay(url: String, state: State<'_, ClientState>) -> Result<(), String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client
        .connect_relay(url, true)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
