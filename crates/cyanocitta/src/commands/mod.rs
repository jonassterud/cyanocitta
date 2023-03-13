use super::AppState;
use anyhow::anyhow;
use nostr::prelude::*;
use secp256k1::{KeyPair, Secp256k1};
use tauri::State;

fn x(err: impl std::fmt::Display) -> String {
    format!("{err}")
}

/// Set secret key.
#[tauri::command]
pub async fn set_secret_key(sk: String, state: State<'_, AppState>) -> Result<(), String> {
    let keys = KeyPair::from_seckey_str(&Secp256k1::new(), &sk).map_err(x)?;
    let mut inner = state.get_inner().await;
    inner.client.keys = keys;

    Ok(())
}

/// Get secret key.
#[tauri::command]
pub async fn get_secret_key(state: State<'_, AppState>) -> Result<String, String> {
    let inner = state.get_inner().await;
    let secret_key = inner.client.keys.display_secret().to_string();

    Ok(secret_key)
}

/// Get public key.
#[tauri::command]
pub async fn get_public_key(state: State<'_, AppState>) -> Result<String, String> {
    let inner = state.get_inner().await;
    let public_key = inner.client.keys.public_key().to_string();

    Ok(public_key)
}

/// Returns whether the app state was created from saved local data.
#[tauri::command]
pub async fn is_from_save(state: State<'_, AppState>) -> Result<bool, String> {
    let inner = state.get_inner().await;
    let is_from_save = inner.from_save;

    Ok(is_from_save)
}

/// Set metadata.
#[tauri::command]
pub async fn set_metadata(metadata: Metadata, state: State<'_, AppState>) -> Result<(), String> {
    let mut inner = state.get_inner().await;
    let existing_metadata = &mut inner.client.metadata;

    inner.client.metadata = Metadata {
        name: metadata.name.or(existing_metadata.name.clone()),
        about: metadata.about.or(existing_metadata.about.clone()),
        picture: metadata.picture.or(existing_metadata.picture.clone()),
    };

    Ok(())
}

/// Get metadata.
#[tauri::command]
pub async fn get_metadata(state: State<'_, AppState>) -> Result<Metadata, String> {
    let inner = state.get_inner().await;
    let metadata = inner.client.metadata.clone();

    Ok(metadata)
}

/// Add relay.
#[tauri::command]
pub async fn add_relay(url: RelayUrl, state: State<'_, AppState>) -> Result<(), String> {
    let mut inner = state.get_inner().await;
    let relay = Relay::new(url);
    inner.client.add_relay(relay);

    Ok(())
}

/// Remove relay.
#[tauri::command]
pub async fn remove_relay(url: RelayUrl, state: State<'_, AppState>) -> Result<(), String> {
    let mut inner = state.get_inner().await;
    inner.client.relays.remove(&url);

    Ok(())
}

/// Start relay listener.
#[tauri::command]
pub async fn listen_relay(url: RelayUrl, buffer: usize, state: State<'_, AppState>) -> Result<(), String> {
    let mut inner = state.get_inner().await;
    let relay = inner.client.relays.get_mut(&url).ok_or_else(|| anyhow!("missing relay")).map_err(x)?;
    relay.listen(buffer).await.map_err(x)?;

    Ok(())
}

/// Try starting listener for all relays (ignoring errors).
#[tauri::command]
pub async fn try_listen_all_relays(buffer: usize, state: State<'_, AppState>) -> Result<(), String> {
    let mut inner = state.get_inner().await;

    for relay in inner.client.relays.values_mut() {
        let _ = relay.listen(buffer).await;
    }

    Ok(())
}

/// Get a list of tuples with relay url and relay status.
#[tauri::command]
pub async fn get_relays(state: State<'_, AppState>) -> Result<Vec<(String, bool)>, String> {
    let inner = state.get_inner().await;
    let relays = inner.client.relays.values().map(|relay| (relay.url.clone(), relay.active)).collect();

    Ok(relays)
}

#[tauri::command]
pub async fn _save_state(state: State<'_, AppState>) -> Result<(), String> {
    state.try_save().await.map_err(x)?;

    Ok(())
}
