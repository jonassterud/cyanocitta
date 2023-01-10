use std::sync::{Arc, Mutex};
use anyhow::Result;
use cyanocitta_nostr_tools::{AppData, Message, Close};
use tauri::Manager;

#[tauri::command]
pub async fn stop_subscription(
    subscription_id: String,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let message = Message::Close(Close::new(subscription_id));
    let app_data = handle.state::<Arc<Mutex<AppData>>>().clone();
    let mut app_data = app_data.lock().map_err(|x| x.to_string())?;
    app_data.message_pool.push(message);
    app_data.save().map_err(|x| x.to_string())?;

    Ok(())
}
