use anyhow::Result;
use secp256k1::{
    hashes::{sha256, Hash},
    Message, Secp256k1, SecretKey,
};
use serde::Serialize;

/// Event.
#[derive(Debug, Serialize)]
pub struct Event {
    /// 32-bytes sha256 of the the serialized event data.
    pub id: Vec<u8>,
    /// 32-bytes hex-encoded public key of the event creator.
    pub pubkey: Vec<u8>,
    /// UNIX timestamp in seconds.
    pub created_at: i64,
    /// Event kind.
    pub kind: u32,
    /// Event tags.
    pub tags: Vec<Vec<String>>,
    /// Arbitrary string.
    pub content: String,
    /// 64-bytes signature of the sha256 hash of the serialized event data, which is the same as the "id" field.
    pub sig: Vec<u8>,
}

impl Event {
    /// Create [`Event`].
    ///
    /// # Arguments
    ///
    /// * `pubkey` - 32-bytes hex-encoded public key of the event creator.
    /// * `created_at` - UNIX timestamp in seconds.
    /// * `kind` - event kind.
    /// * `tags` - event tags.
    /// * `content` - arbitrary string.
    /// * `secret_key` - [`SecretKey`] for `pubkey`.
    pub fn new(
        pubkey: Vec<u8>,
        created_at: i64,
        kind: u32,
        tags: Vec<Vec<String>>,
        content: String,
        secret_key: SecretKey,
    ) -> Result<Self> {
        let id = sha256::Hash::hash(
            serde_json::json!([0, pubkey, created_at, kind, tags, content])
                .to_string()
                .as_bytes(),
        )
        .to_vec();

        let sig = Secp256k1::new()
            .sign_ecdsa(&Message::from_hashed_data::<sha256::Hash>(&id), &secret_key)
            .serialize_compact()
            .to_vec();

        Ok(Self {
            id,
            pubkey,
            created_at,
            kind,
            tags,
            content,
            sig,
        })
    }
}
