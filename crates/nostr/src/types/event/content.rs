use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Nostr event content.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct EventContent(pub String);

#[derive(Deserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub about: String,
    pub picture: String,
}
