use anyhow::Result;
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
    pub tags: (char, Vec<String>),
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
    pub fn new(
        pubkey: Vec<u8>,
        created_at: i64,
        kind: u32,
        tags: (char, Vec<String>),
        content: String,
    ) -> Result<Self> {
        Ok(Self {
            id: todo!(),
            pubkey,
            created_at,
            kind,
            tags,
            content,
            sig: todo!(),
        })
    }
}
