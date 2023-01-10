use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::{AppData, Message, Req, Filters};
use tauri::State;

#[tauri::command]
pub fn start_subscription(
    subscription_id: String,
    app_data: State<Mutex<AppData>>,
) -> Result<(), String> {
    let message = Message::Req(Req::new(subscription_id, Filters::default()));
    app_data.lock().map_err(|x| x.to_string())?.message_pool.push(message);

    Ok(())
}
