use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::{AppData, Profile};
use secp256k1::SecretKey;
use tauri::State;

#[tauri::command]
pub fn load_subscription(app_data: State<Mutex<AppData>>) -> Result<String, String> {
    todo!()
}
