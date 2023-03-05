//! Nostr types for events.

mod content;
mod id;
mod kind;
mod sig;
mod tags;
mod timestamp;

pub use content::EventContent;
pub use id::EventId;
pub use kind::EventKind;
pub use sig::EventSig;
pub use tags::EventTag;
pub use timestamp::EventTimestamp;

use anyhow::Result;
use secp256k1::{KeyPair, XOnlyPublicKey};
use serde::{Deserialize, Serialize};

/// Nostr event object.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures
#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub id: Option<EventId>,
    pub pubkey: XOnlyPublicKey,
    pub created_at: EventTimestamp,
    pub kind: EventKind,
    pub tags: Vec<EventTag>,
    pub content: EventContent,
    pub sig: Option<EventSig>,
}

impl Event {
    /// Create [`Event`].
    pub fn new(keys: &KeyPair, kind: EventKind, tags: Vec<EventTag>, content: EventContent) -> Self {
        let event = Self { id: None, pubkey: keys.x_only_public_key().0, created_at: 0, kind, tags, content, sig: None };

        event.update_id()
    }

    /// Update [`EventId`] for [`Event`].
    fn update_id(self) -> Self {
        Self { id: Some(EventId::generate(&self)), ..self }
    }

    /// Sign [`Event`].
    pub fn sign(self, keys: &KeyPair) -> Result<Self> {
        Ok(Self { sig: Some(EventSig::generate(&self, keys)?), ..self })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{rand, KeyPair, Secp256k1};

    #[test]
    pub fn create_event() {
        let secp = Secp256k1::new();
        let keys = KeyPair::new(&secp, &mut rand::thread_rng());
        let event = Event::new(&keys, EventKind::ShortTextNote, vec![], "test".to_string());
        let event = event.sign(&keys).unwrap();

        println!("{:?}", event);
    }

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
