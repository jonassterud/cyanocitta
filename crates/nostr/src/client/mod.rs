//! Nostr client.

mod relay;
mod send;
mod listen;

use relay::{Relay, RelayUrl};
use std::collections::HashMap;
use secp256k1::{rand, KeyPair, Secp256k1, SecretKey};

/// Nostr client to interact with relays.
pub struct Client {
    pub keys: KeyPair,
    pub relays: HashMap<RelayUrl, Relay>,
}

impl Client {
    /// Create [`Client`] from keys.
    pub fn from_keys(keys: KeyPair) -> Self {
        Self { keys, relays: HashMap::new() }
    }

    /// Create [`Client`] from randomly generated keys.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::new(&secp, &mut rand::thread_rng());

        Self::from_keys(keys)
    }

    /// Create [`Client`] from secret key.
    pub fn from_secret_key(sk: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::from_secret_key(&secp, &sk);

        Self::from_keys(keys)
    }
}
