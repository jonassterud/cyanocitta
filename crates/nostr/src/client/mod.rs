//! Nostr client.

use secp256k1::{rand, Secp256k1, SecretKey, XOnlyPublicKey};

/// Nostr client to interact with relays.
pub struct Client {
    pub public_key: XOnlyPublicKey,
    secret_key: SecretKey,
}

impl Client {
    /// Create new [`Client`].
    pub fn new() -> Client {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        let (x_public_key, _) = public_key.x_only_public_key();

        Client {
            public_key: x_public_key,
            secret_key,
        }
    }
}
