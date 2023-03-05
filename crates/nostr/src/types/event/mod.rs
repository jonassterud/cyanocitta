//! Nostr types for events.

mod content;
mod id;
mod kind;
mod sig;
mod tags;

pub use content::EventContent;
pub use id::EventId;
pub use kind::EventKind;
pub use sig::EventSig;
pub use tags::EventTag;

use secp256k1::XOnlyPublicKey;
use serde::{Deserialize, Serialize};

/// Nostr event object.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures
#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub id: EventId,
    pub pubkey: XOnlyPublicKey,
    pub created_at: u32,
    pub kind: EventKind,
    pub tags: Vec<EventTag>,
    pub content: EventContent,
    pub sig: EventSig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{rand, KeyPair, Secp256k1};

    #[test]
    pub fn serialize_event() {
        // KeyPair::new(&Secp256k1::new(), &mut rand::thread_rng())
        todo!()
    }

    #[test]
    pub fn deserialize_event() {
        todo!()
    }
}
