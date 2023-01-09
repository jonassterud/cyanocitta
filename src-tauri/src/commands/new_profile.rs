use std::sync::Mutex;

use anyhow::Result;
use cyanocitta_nostr_tools::{AppData, Profile};
use secp256k1::SecretKey;
use tauri::State;

#[tauri::command]
pub fn new_profile(secret: Option<String>, app_data: State<Mutex<AppData>>) -> Result<(), String> {
    let profile = if let Some(sk) = secret {
        let sk_bytes = SecretKey::from_slice(sk.as_bytes()).map_err(|x| x.to_string())?;
        Profile::from_secret_key(sk_bytes)
    } else {
        Profile::new_with_random_keypair()
    };

    app_data
        .lock()
        .map_err(|x| x.to_string())?
        .profiles
        .push(profile);

    Ok(())
}
