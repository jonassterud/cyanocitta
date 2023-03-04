//! Nostr client.

use secp256k1::{rand, KeyPair, Secp256k1, SecretKey};

/// Nostr client to interact with relays.
pub struct Client {
    pub keys: KeyPair,
}

impl Client {
    /// Create new [`Client`] from randomly generated keys.
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::new(&secp, &mut rand::thread_rng());

        Self { keys }
    }

    /// Create [`Client`] from secret key.
    pub fn from_secret_key(sk: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let keys = KeyPair::from_secret_key(&secp, &sk);

        Self { keys }
    }
}
