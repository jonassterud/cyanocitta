use anyhow::Result;
use secp256k1::{
    hashes::{sha256, Hash},
    Message, Secp256k1, SecretKey,
};
use serde::{Deserialize, Serialize};

/// An event is the only object type sent over the Nostr network.
#[derive(Deserialize, Serialize)]
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
    /// * `pubkey` - [`PublicKey`] as a vector.
    /// * `seckey` - [`SecretKey`] as a vector.
    /// * `kind` - event kind.
    /// * `tags` - event tags.
    /// * `content` - arbitrary string.
    fn new(
        pubkey: &[u8],
        seckey: &[u8],
        kind: u32,
        tags: Vec<Vec<String>>,
        content: String,
    ) -> Result<Self> {
        let pubkey = pubkey[1..].iter().map(|x| format!("{x:x}")).collect();
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;
        let id = sha256::Hash::hash(
            serde_json::json!([0, pubkey, created_at, kind, tags, content])
                .to_string()
                .as_bytes(),
        )
        .to_string();
        let sig = Secp256k1::new()
            .sign_ecdsa(
                &Message::from_hashed_data::<sha256::Hash>(id.as_bytes()),
                &SecretKey::from_slice(seckey)?,
            )
            .serialize_compact()
            .iter()
            .map(|x| format!("{x:x}"))
            .collect();

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

    /// Create [`Event`] with kind `0 (set_metadata)`.
    ///
    /// # Arguments
    ///
    /// * `pubkey` - [`PublicKey`] as a vector.
    /// * `seckey` - [`SecretKey`] as a vector.
    /// * `name` - username.
    /// * `about` - user description.
    /// * `picture` - image url.
    pub fn new_set_metadata(
        pubkey: &[u8],
        seckey: &[u8],
        name: &str,
        about: &str,
        picture: &str,
    ) -> Result<Self> {
        Self::new(
            pubkey,
            seckey,
            0,
            vec![],
            serde_json::json!({
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
    /// * `pubkey` - [`PublicKey`] as a vector.
    /// * `seckey` - [`SecretKey`] as a vector.
    /// * `content` - note (or other stuff).
    pub fn new_text_note(pubkey: &[u8], seckey: &[u8], content: String) -> Result<Self> {
        Self::new(pubkey, seckey, 1, vec![], content)
    }

    /// Create [`Event`] with kind `2 (recommend_server)`.
    ///
    /// # Arguments
    ///
    /// * `pubkey` - [`PublicKey`] as a vector.
    /// * `seckey` - [`SecretKey`] as a vector.
    /// * `url` - recommended server url.
    pub fn new_recommend_server(pubkey: &[u8], seckey: &[u8], url: String) -> Result<Self> {
        Self::new(pubkey, seckey, 2, vec![], url)
    }
}
