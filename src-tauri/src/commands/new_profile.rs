use async_std::sync::{Arc, Mutex};
use anyhow::Result;
use cyanocitta_nostr_tools::{AppData, Profile};
use secp256k1::SecretKey;
use tauri::Manager;

#[tauri::command]
pub async fn new_profile(secret: Option<String>, handle: tauri::AppHandle) -> Result<(), String> {
    let profile = if let Some(sk) = secret {
        let sk_bytes = SecretKey::from_slice(sk.as_bytes()).map_err(|x| x.to_string())?;
        Profile::from_secret_key(sk_bytes)
    } else {
        Profile::new_with_random_keypair()
    };

    let app_data = handle.state::<Arc<Mutex<AppData>>>();
    app_data
        .lock().await
        .profiles
        .push(profile);
    app_data.lock().await.save().map_err(|x| x.to_string())?;

    Ok(())
}
