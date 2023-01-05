use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::{Client, Profile};
use secp256k1::SecretKey;
use tauri::State;

#[tauri::command]
pub fn refresh_subscription(client: State<Mutex<Client>>) -> Result<String, String> {
    todo!()
}
