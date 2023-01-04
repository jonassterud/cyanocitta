use anyhow::Result;
use cyanocitta_nostr_tools::Profile;
use secp256k1::SecretKey;

#[tauri::command]
pub fn new_profile(secret_key: Option<String>) -> Result<String, String> {
    if let Some(sk) = secret_key {
        let sk_bytes = SecretKey::from_slice(sk.as_bytes()).map_err(|x| x.to_string())?;
        Profile::from_secret_key(sk_bytes)
            .as_json()
            .map_err(|x| x.to_string())
    } else {
        Profile::new_with_random_keypair()
            .as_json()
            .map_err(|x| x.to_string())
    }
}
