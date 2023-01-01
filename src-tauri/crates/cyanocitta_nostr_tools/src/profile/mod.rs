use secp256k1::{rand::rngs::OsRng, PublicKey, Secp256k1, SecretKey};

#[derive(Debug)]
pub struct Profile {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
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
