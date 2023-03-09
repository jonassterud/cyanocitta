use crate::types::Event;
use anyhow::{anyhow, Result};
use secp256k1::hashes::{hex::ToHex, sha256, Hash};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Nostr event id.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct EventId(pub String);

impl EventId {
    /// Generate [`EventId`] for [`Event`].
    pub fn generate(event: &Event) -> Self {
        let json = json!([
            0,
            event.pubkey.to_string(),
            event.created_at,
            event.kind as u64,
            event.tags,
            event.content.0
        ])
        .to_string();
        let hash = sha256::Hash::hash(json.as_bytes()).to_hex();

        Self(hash)
    }

    /// Verify [`EventId`].
    pub fn verify(&self, event: &Event) -> Result<()> {
        match &Self::generate(event) == self {
            true => Ok(()),
            false => Err(anyhow!("id's don't match")),
        }
    }
}
