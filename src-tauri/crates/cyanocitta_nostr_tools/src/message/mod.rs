mod close;
mod event;
mod req;

use anyhow::{anyhow, Result};
pub use close::*;
pub use event::*;
pub use req::*;
use serde_json::json;

/// Message.
#[derive(Debug)]
pub enum Message {
    /// See [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures).
    Event(Event),
    /// Used to request events and subscribe to new updates.
    Req(Req),
    /// Used to stop previous subscriptions.
    Close(Close),
    /// Used to send human-readable error messages or other things to **clients**.
    Notice(String),
}

impl Message {
    /// Serialize [`Message`] into JSON.
    pub fn serialize(&self) -> String {
        match self {
            Message::Event(event) => json!(["EVENT", event]).to_string(),
            Message::Req(req) => json!(["REQ", req.subscription_id, req.filters]).to_string(),
            Message::Close(close) => json!(["CLOSE", close.subscription_id]).to_string(),
            Message::Notice(string) => json!(["NOTICE", string]).to_string(),
        }
    }

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
