use crate::client_state::ClientState;
use nostr_sdk::prelude::*;
use tauri::State;

/// Follow pk.
///
/// # Errors
///
/// This function will never return an error.
#[tauri::command]
pub async fn follow(pk: XOnlyPublicKey, state: State<'_, ClientState>) -> Result<(), String> {
    let mut inner = state.0.lock().await;
    inner.following.insert(pk);

    Ok(())
}

/// Unfollow pk.
///
/// # Errors
///
/// This function will never return an error.
#[tauri::command]
pub async fn unfollow(pk: XOnlyPublicKey, state: State<'_, ClientState>) -> Result<(), String> {
    let mut inner = state.0.lock().await;
    inner.following.remove(&pk);

    Ok(())
}

/// Get following.
///
/// # Errors
///
/// This function will return an error if:
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn get_following(state: State<'_, ClientState>) -> Result<String, String> {
    let following = &state.0.lock().await.following;
    let json = serde_json::to_string(following).map_err(|e| e.to_string())?;

    Ok(json)
}

/// Check if following pk.
///
/// # Errors
///
/// This function will return an error if:
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn is_following(
    pk: XOnlyPublicKey,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let is_following = &state.0.lock().await.following.contains(&pk);
    let json = serde_json::to_string(is_following).map_err(|e| e.to_string())?;

    Ok(json)
}
