use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::{AppData, Message, Close};
use tauri::State;

#[tauri::command]
pub fn stop_subscription(
    subscription_id: String,
    app_data: State<Mutex<AppData>>,
) -> Result<(), String> {
    let message = Message::Close(Close::new(subscription_id));
    app_data.lock().map_err(|x| x.to_string())?.message_pool.push(message);

    Ok(())
}
