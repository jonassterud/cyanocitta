use anyhow::Result;
use secp256k1::{
    hashes::{sha256, Hash},
    Message, PublicKey, Secp256k1, SecretKey,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

/// Event.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Event {
    /// 32-bytes sha256 of the the serialized event data.
    pub id: String,
    /// 32-bytes hex-encoded public key of the event creator.
    pub pubkey: String,
    /// UNIX timestamp in seconds.
    pub created_at: i64,
    /// Event kind.
    pub kind: u32,
    /// Event tags.
    pub tags: Vec<Vec<String>>,
    /// Arbitrary string.
    pub content: String,
    /// 64-bytes signature of the sha256 hash of the serialized event data, which is the same as the "id" field.
    pub sig: String,
}

impl Event {
    /// Create [`Event`].
    ///
    /// # Arguments
    ///
    /// * `public_key` - [`PublicKey`].
    /// * `secret_key` - [`SecretKey`].
    /// * `created_at` - UNIX timestamp in seconds.
    /// * `kind` - event kind.
    /// * `tags` - event tags.
    /// * `content` - arbitrary string.
    pub fn new(
        public_key: PublicKey,
        secret_key: SecretKey,
        created_at: i64,
        kind: u32,
        tags: Vec<Vec<String>>,
        content: String,
    ) -> Result<Self> {
        let id = sha256::Hash::hash(
            serde_json::json!([
                0,
                String::from_utf8(public_key.serialize()[1..].to_vec())?,
                created_at,
                kind,
                tags,
                content
            ])
            .to_string()
            .as_bytes(),
        )
        .to_string();

        let sig = String::from_utf8(
            Secp256k1::new()
                .sign_ecdsa(
                    &Message::from_hashed_data::<sha256::Hash>(id.as_bytes()),
                    &secret_key,
                )
                .serialize_compact()
                .to_vec(),
        )?;

        Ok(Self {
            id,
            pubkey: public_key.to_string(),
            created_at,
            kind,
            tags,
            content,
            sig,
        })
    }

    /// Create [`Event`] with kind `0 (set_metadata)`.
    ///
    /// # Arguments
    ///
    /// * `public_key` - [`PublicKey`].
    /// * `secret_key` - [`SecretKey`].
    /// * `name` - username.
    /// * `about` - user description.
    /// * `picture` - image url.
    pub fn new_set_metadata(
        public_key: PublicKey,
        secret_key: SecretKey,
        name: &str,
        about: &str,
        picture: &str,
    ) -> Result<Self> {
        Self::new(
            public_key,
            secret_key,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            0,
            vec![],
            json!({
                "name": name,
                "about": about,
                "picture": picture
            })
            .to_string(),
        )
    }

    /// Create [`Event`] with kind `1 (text_note)`.
    ///
    /// # Arguments
    ///
    /// * `public_key` - [`PublicKey`].
    /// * `secret_key` - [`SecretKey`].
    /// * `content` - note (or other stuff).
    pub fn new_text_note(
        public_key: PublicKey,
        secret_key: SecretKey,
        content: String,
    ) -> Result<Self> {
        Self::new(
            public_key,
            secret_key,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            1,
            vec![],
            content,
        )
    }

    /// Create [`Event`] with kind `2 (recommend_server)`.
    ///
    /// # Arguments
    ///
    /// * `public_key` - [`PublicKey`].
    /// * `secret_key` - [`SecretKey`].
    /// * `url` - recommended server url.
    pub fn new_recommend_server(
        public_key: PublicKey,
        secret_key: SecretKey,
        url: String,
    ) -> Result<Self> {
        Self::new(
            public_key,
            secret_key,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            2,
            vec![],
            url,
        )
    }
}
