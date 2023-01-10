mod close;
mod event;
mod req;

use anyhow::Result;
pub use close::*;
pub use event::*;
pub use req::*;
use serde::{
    de::{self, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

/// Message.
#[derive(Debug)]
#[repr(usize)]
pub enum Message {
    /// See [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures).
    Event(Event) = 0,
    /// Used to request events and subscribe to new updates.
    Req(Req) = 1,
    /// Used to stop previous subscriptions.
    Close(Close) = 2,
    /// Used to send human-readable error messages or other things to **clients**.
    Notice(String) = 3,
}

struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "NIP-01 compatible json")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let message_type = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Missing message type."))?;

        match message_type.as_str() {
            "EVENT" => {
                let event = seq
                    .next_element::<Event>()?
                    .ok_or_else(|| de::Error::custom("Missing event."))?;

                Ok(Message::Event(event))
            }
            "REQ" => {
                let subscription_id = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("Missing subscription id."))?;
                let filters = seq
                    .next_element::<Filters>()?
                    .ok_or_else(|| de::Error::custom("Missing filters."))?;
                let req = Req::new(subscription_id, filters);

                Ok(Message::Req(req))
            }
            "CLOSE" => {
                let subscription_id = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("Missing subscription id."))?;
                let close = Close::new(subscription_id);

                Ok(Message::Close(close))
            }
            "NOTICE" => {
                let notice_message = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::custom("Missing notice message."))?;

                Ok(Message::Notice(notice_message))
            }
            _ => return Err(de::Error::custom("Unknown message type.")),
        }
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(MessageVisitor)
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Message::Event(data) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("EVENT")?;
                seq.serialize_element(&data)?;
                seq.end()
            }
            Message::Req(data) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element("REQ")?;
                seq.serialize_element(&data.subscription_id)?;
                seq.serialize_element(&data.filters)?;
                seq.end()
            }
            Message::Close(data) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("CLOSE")?;
                seq.serialize_element(&data.subscription_id)?;
                seq.end()
            }
            Message::Notice(data) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("NOTICE")?;
                seq.serialize_element(data)?;
                seq.end()
            }
        }
    }
}
