use secp256k1::hashes::sha256;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1};
use secp256k1::{PublicKey, SecretKey};

#[derive(Debug)]
pub struct Profile {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl Profile {
    /// Create [`Profile`] with random keypair.
    pub fn new_with_random_keypair() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        Self {
            secret_key,
            public_key,
        }
    }
}
