//! Nostr types for events.

mod id;
mod kind;
mod sig;
mod tags;

pub use id::EventId;
pub use kind::EventKind;
pub use sig::EventSig;
pub use tags::{EventTag, EventTags};

use secp256k1::XOnlyPublicKey;
use serde::{Deserialize, Serialize};

/// Nostr event object.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures
#[derive(Deserialize, Serialize)]
pub struct Event {
    pub id: EventId,
    pub pubkey: XOnlyPublicKey,
    pub created_at: u32,
    pub kind: EventKind,
    pub tags: EventTags,
    pub content: String,
    pub sig: EventSig,
}
