use crate::client_state::ClientState;
use anyhow::anyhow;
use nostr_sdk::prelude::*;
use tauri::State;

/// Subscribe to filters.
///
/// # Errors
///
/// This function will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
/// * `serde_json` serialization fails.
#[tauri::command]
pub async fn subscribe(
    filters: Vec<Filter>,
    state: State<'_, ClientState>,
) -> Result<String, String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;
    let filters_json = serde_json::to_vec(&filters).map_err(|e| e.to_string())?;
    let hash = sha256::Hash::hash(&filters_json).to_string();
    let subscription_id = SubscriptionId::new(&hash[..32]);
    let out_json = serde_json::to_string(&subscription_id).map_err(|e| e.to_string())?;

    client
        .send_msg(ClientMessage::new_req(subscription_id, filters))
        .await
        .map_err(|e| e.to_string())?;

    Ok(out_json)
}

/// Unsubscribe.
///
/// # Errors
///
/// This function will return an error if:
/// * `client` in [`InnerClientState`] is `None`.
/// * [`Client::send_msg`] fails.
#[tauri::command]
pub async fn unsubscribe(
    subscription_id: SubscriptionId,
    state: State<'_, ClientState>,
) -> Result<(), String> {
    let inner = state.0.lock().await;
    let client = inner
        .client
        .as_ref()
        .ok_or_else(|| anyhow!("missing client").to_string())?;

    client
        .send_msg(ClientMessage::close(subscription_id))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
