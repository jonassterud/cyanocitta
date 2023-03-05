use super::id::EventId;
use secp256k1::XOnlyPublicKey;
use serde::{
    de::{self, Deserialize, Visitor},
    ser::{Serialize, Serializer},
};

/// Nostr event tag.
///
/// https://github.com/nostr-protocol/nips#standardized-tags
#[derive(PartialEq, Eq, Debug)]
#[repr(usize)]
pub enum EventTag {
    // ["e", "<32-bytes hex of the id of another event>", "<relay url>]"]
    E { event_id: EventId, relay_url: String, marker: String },
    // // ["p", "<32-bytes hex of a pubkey>", "<relay url>"]
    P { pubkey: XOnlyPublicKey, relay_url: String },
    // ["a", "<kind>:<pubkey>:<d-identifier>", "<relay url>"]
    A { coordinates: String, relay_url: String },
    // ["r", "<a reference (URL, etc)>"]
    R { reference: String },
    // ["t", "<hashtag>"]
    T { hashtag: String },
    //  ["g", "<geohash>"]
    G { geohash: String },
    // ["nonce", "<random nonce>", "<target difficulty>"]
    Nonce { nonce: String, target: String },
    // ["subject", "<subject>"]
    Subject { subject: String },
    // ["d", "<identifier>"]
    D { identifier: String },
    // ["expiration", "<UNIX timestamp>"]
    Expiration { timestamp: String },
}

struct EventTagVisitor;

impl<'de> Visitor<'de> for EventTagVisitor {
    type Value = EventTag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let tag_name = seq.next_element::<String>()?.ok_or(de::Error::custom("missing tag name"))?;
        match &*tag_name {
            "e" => Ok(Self::Value::E {
                event_id: seq.next_element()?.ok_or(de::Error::custom("missing event id"))?,
                relay_url: seq.next_element()?.unwrap_or_default(),
                marker: seq.next_element()?.unwrap_or_default(),
            }),
            "p" => Ok(Self::Value::P {
                pubkey: seq.next_element()?.ok_or(de::Error::custom("missing pubkey"))?,
                relay_url: seq.next_element()?.unwrap_or_default(),
            }),
            "a" => Ok(Self::Value::A {
                coordinates: seq.next_element::<String>()?.ok_or(de::Error::custom("missing coordinates"))?,
                relay_url: seq.next_element()?.unwrap_or_default(),
            }),
            "r" => Ok(Self::Value::R { reference: seq.next_element::<String>()?.ok_or(de::Error::custom("missing reference"))? }),
            "t" => Ok(Self::Value::T { hashtag: seq.next_element::<String>()?.ok_or(de::Error::custom("missing hashtag"))? }),
            "g" => Ok(Self::Value::G { geohash: seq.next_element::<String>()?.ok_or(de::Error::custom("missing geohash"))? }),
            "nonce" => Ok(Self::Value::Nonce {
                nonce: seq.next_element::<String>()?.ok_or(de::Error::custom("missing nonce"))?,
                target: seq.next_element::<String>()?.unwrap_or_default(),
            }),
            "subject" => Ok(Self::Value::Subject { subject: seq.next_element::<String>()?.ok_or(de::Error::custom("missing subject"))? }),
            "d" => Ok(Self::Value::D { identifier: seq.next_element::<String>()?.ok_or(de::Error::custom("missing identifier"))? }),
            "expiration" => Ok(Self::Value::Expiration { timestamp: seq.next_element::<String>()?.ok_or(de::Error::custom("missing timestamp"))? }),
            _ => Err(de::Error::custom("unknown tag name")),
        }
    }
}

impl<'de> Deserialize<'de> for EventTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(EventTagVisitor)
    }
}

impl Serialize for EventTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EventTag::E { event_id, relay_url, marker } => ("e", event_id, relay_url, marker).serialize(serializer),
            EventTag::P { pubkey, relay_url } => ("p", pubkey, relay_url).serialize(serializer),
            EventTag::A { coordinates, relay_url } => ("a", coordinates, relay_url).serialize(serializer),
            EventTag::R { reference } => ("r", reference).serialize(serializer),
            EventTag::T { hashtag } => ("t", hashtag).serialize(serializer),
            EventTag::G { geohash } => ("g", geohash).serialize(serializer),
            EventTag::Nonce { nonce, target } => ("nonce", nonce, target).serialize(serializer),
            EventTag::Subject { subject } => ("subject", subject).serialize(serializer),
            EventTag::D { identifier } => ("d", identifier).serialize(serializer),
            EventTag::Expiration { timestamp } => ("expiration", timestamp).serialize(serializer),
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_event_tag_serialization() {
        let pairs = vec![(
            EventTag::E { event_id: "event_id".to_string(), relay_url: "".to_string(), marker: "root".to_string() },
            "[\"e\",\"event_id\",\"\",\"root\"]",
        )];

        for (tag, serialized_tag) in pairs {
            assert_eq!(serde_json::to_string(&tag).unwrap(), serialized_tag);
        }
    }

    #[test]
    pub fn test_event_tag_deserialization() {
        let pairs = vec![(
            EventTag::E { event_id: "event_id".to_string(), relay_url: "".to_string(), marker: "root".to_string() },
            "[\"e\",\"event_id\",\"\",\"root\"]",
        )];

        for (tag, serialized_tag) in pairs {
            assert_eq!(serde_json::from_str::<EventTag>(serialized_tag).unwrap(), tag);
        }
    }
}
*/
