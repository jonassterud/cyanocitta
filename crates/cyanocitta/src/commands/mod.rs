use super::AppState;
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

/// Add relay.
#[tauri::command]
pub async fn add_relay(url: String, buffer: usize, state: State<'_, AppState>) -> Result<(), String> {
    let mut relay = Relay::new(&url);
    relay.listen(buffer).await.map_err(x)?;

    let mut inner = state.get_inner().await;
    inner.client.add_relay(relay);

    Ok(())
}
