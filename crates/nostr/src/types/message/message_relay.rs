use crate::types::{Event, EventId};
use serde::de::{self, Deserialize, Visitor};

/// Message sent from relay to client.
///
/// https://github.com/nostr-protocol/nips#client-to-relay
#[derive(Clone, Debug)]
pub enum RelayMessage {
    Event { subscription_id: String, event: Event },
    Notice { message: String },
    Eose { subscription_id: String },
    Ok { event_id: EventId, status: bool, message: String },
    Auth { challenge_string: String },
}

struct RelayMessageVisitor;

impl<'de> Visitor<'de> for RelayMessageVisitor {
    type Value = RelayMessage;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a JSON sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let event_type = seq.next_element::<String>()?.ok_or(de::Error::custom("missing event type"))?;
        match &*event_type {
            "EVENT" => Ok(Self::Value::Event {
                subscription_id: seq.next_element::<String>()?.ok_or(de::Error::custom("missing subscription id"))?,
                event: seq.next_element::<Event>()?.ok_or(de::Error::custom("missing event"))?,
            }),
            "NOTICE" => Ok(Self::Value::Notice {
                message: seq.next_element::<String>()?.ok_or(de::Error::custom("missing message"))?,
            }),
            "EOSE" => Ok(Self::Value::Eose {
                subscription_id: seq.next_element::<String>()?.ok_or(de::Error::custom("missing subscription id"))?,
            }),
            "OK" => Ok(Self::Value::Ok {
                event_id: seq.next_element::<EventId>()?.ok_or(de::Error::custom("missing event id"))?,
                status: seq.next_element::<bool>()?.ok_or(de::Error::custom("missing status"))?,
                message: seq.next_element::<String>()?.ok_or(de::Error::custom("missing message"))?,
            }),
            "AUTH" => Ok(Self::Value::Auth {
                challenge_string: seq.next_element::<String>()?.ok_or(de::Error::custom("missing challenge string"))?,
            }),
            _ => Err(de::Error::custom("unknown event type")),
        }
    }
}

impl<'de> Deserialize<'de> for RelayMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(RelayMessageVisitor)
    }
}
