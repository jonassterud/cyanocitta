use crate::client_state::ClientState;
use nostr_sdk::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn get_pk(state: State<'_, ClientState>) -> Result<String, String> {
    let inner = state.0.lock().await;
    let pk = inner.pk.clone();

    Ok(pk)
}