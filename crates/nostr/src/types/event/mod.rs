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
pub use tags::{EventTag, EventTags};

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
    pub tags: EventTags,
    pub content: EventContent,
    pub sig: EventSig,
}
#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{rand, KeyPair, Secp256k1, SecretKey};

    #[test]
    pub fn test_event_serialization() {
        let secp = Secp256k1::new();
        let keys = KeyPair::new(&secp, &mut rand::thread_rng());

        let left = Event {
            id: "event_id_0".to_string(),
            pubkey: keys.x_only_public_key().0,
            created_at: 0,
            kind: EventKind::Metadata,
            tags: vec![EventTag::E { event_id: "event_id_1".to_string(), relay_url: "".to_string(), marker: "".to_string() }],
            content: EventContent::Metadata { name: Some("name".to_string()), about: None, picture: None },
            sig: "signature".to_string(),
        };

        let right = "{
            \"id\": \"event_id_0\",
            \"pubkey\": \"295c92f5b9cd01e3b856caf76dcd9c3308124c04774744cf353d92e8402a97b1\",
            \"created_at\": 0,
            \"kind\": 0,
            \"tags\": [
              [
                \"e\",
                \"event_id_1\",
                \"\",
                \"\"
              ]
            ],
            \"content\": {
              \"name\": \"name\"
            },
            \"sig\": \"signature\"
          }";

        let serialized_event = serde_json::to_string_pretty(&left).unwrap();
        let deserialized_event = serde_json::from_str::<Event>(right).unwrap();
        println!("{serialized_event}");
        println!("{:?}", deserialized_event)
    }
}
