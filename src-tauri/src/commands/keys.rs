use crate::client_state::{ClientState, InnerClientState};
use nostr_sdk::prelude::*;
use tauri::State;

#[tauri::command]
pub async fn get_my_pk(state: State<'_, ClientState>) -> Result<String, String> {
    let inner = state.0.lock().await;
    let pk = inner.pk.to_string();

    Ok(pk)
}

#[tauri::command]
pub async fn set_new_sk(sk: String, state: State<'_, ClientState>) -> Result<(), String> {
    let new_inner_client_state = InnerClientState::from_sk(&sk).map_err(|e| e.to_string())?;

    *state.0.lock().await = new_inner_client_state;
    state.initialize_client().await.map_err(|e| e.to_string())?;

    Ok(())
}
