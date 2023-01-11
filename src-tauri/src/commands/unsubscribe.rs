use crate::nostr_tools::*;
use anyhow::Result;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn unsubscribe(subscription_id: String, handle: tauri::AppHandle) -> Result<(), String> {
    let message = Message::Close(subscription_id);
    let message_json = serde_json::to_string(&message).map_err(|x| x.to_string())?;
    handle
        .state::<Arc<Mutex<Client>>>()
        .lock()
        .await
        .pool
        .push_back(message_json);

    Ok(())
}
