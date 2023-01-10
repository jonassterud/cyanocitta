use async_std::sync::{Arc, Mutex};
use anyhow::Result;
use cyanocitta_nostr_tools::{AppData, Message, Req, Filters};
use tauri::Manager;

#[tauri::command]
pub async fn start_subscription(
    subscription_id: String,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let message = Message::Req(Req::new(subscription_id, Filters::default()));
    let app_data = handle.state::<Arc<Mutex<AppData>>>();
    app_data.lock().await.message_pool.push(message);
    app_data.lock().await.save().map_err(|x| x.to_string())?;
    
    Ok(())
}
