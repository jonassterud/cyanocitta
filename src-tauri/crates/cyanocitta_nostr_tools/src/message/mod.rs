mod close;
mod event;
mod req;

use anyhow::{anyhow, Result};
pub use close::*;
pub use event::*;
pub use req::*;
use serde::{
    de::{self, Visitor},
    ser::{SerializeSeq, SerializeStructVariant},
    Deserialize, Serialize,
};
use serde_json::json;

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

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {   
        todo!()
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

/*
impl Message {
    /// Deserialize JSON into [`Message`].
    ///
    /// # Arguments
    ///
    /// * `json` - JSON string according to [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md#from-relay-to-client-sending-events-and-notices).
    pub fn deserialize(json: &str) -> Result<Message> {
        use serde_json as sjson;

        let json = sjson::from_str::<sjson::Value>(&json)?;
        let json = json
            .as_array()
            .ok_or_else(|| anyhow!("should be an array"))?;

        match json.get(0).map(|x| x.as_str()).flatten() {
            Some("EVENT") => {
                let subscription_id = json
                    .get(1)
                    .ok_or_else(|| anyhow!("mising \"subscription_id\""))?
                    .to_owned();
                let subscription_id: String = sjson::from_value(subscription_id)?;
                let event: Event = serde_json::from_value(
                    json.get(2)
                        .ok_or_else(|| anyhow!("missing \"event\""))?
                        .to_owned(),
                )?;

                Ok(Message::Event(event))
            }
            Some("NOTICE") => {
                let message: String = serde_json::from_value(
                    json.get(1)
                        .ok_or_else(|| anyhow!("missing \"message\""))?
                        .to_owned(),
                )?;

                Ok(Message::Notice(message))
            }
            _ => Err(anyhow!("array should start with \"EVENT\" or \"NOTICE\"")),
        }
    }
}
 */
