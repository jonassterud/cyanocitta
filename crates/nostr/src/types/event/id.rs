use crate::types::Event;
use secp256k1::hashes::{hex::ToHex, sha256, Hash};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Nostr event id.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct EventId(pub String);

impl EventId {
    pub fn generate(event: &Event) -> Self {
        let json = json!([0, event.pubkey.to_string(), event.created_at, event.kind as u64, event.tags, event.content]).to_string();
        let hash = sha256::Hash::hash(json.as_bytes()).to_hex();

        Self(hash)
    }
}
