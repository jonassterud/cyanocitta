mod close;
mod event;
mod req;

use anyhow::{anyhow, Result};
pub use close::*;
pub use event::*;
pub use req::*;
use serde_json::json;

#[derive(Debug)]
pub enum Message {
    Event(Event),
    Req(Req),
    Close(Close),
}

impl Message {
    /// Serialize [`Message`] into JSON.
    pub fn serialize(&self) -> Result<String> {
        match self {
            Message::Event(event) => Ok(serde_json::json!(["EVENT", event]).to_string()),
            Message::Req(req) => {
                Ok(serde_json::json!(["REQ", req.subscription_id, req.filters]).to_string())
            }
            Message::Close(close) => {
                Ok(serde_json::json!(["CLOSE", close.subscription_id]).to_string())
            }
        }
    }

    /// Read [`Message`] from relay.
    pub fn from_relay(json: String) -> Result<Self> {
        let json = serde_json::from_str::<serde_json::Value>(&json)?;
        let json = json.as_array().ok_or_else(|| anyhow!("not an array"))?;

        if json.get(0) != Some(&json!("EVENT")) {
            return Err(anyhow!("expected event, got: {:?}", json));
        }

        let subscription_id = json
            .get(1)
            .map(|x| x.as_str())
            .flatten()
            .ok_or_else(|| anyhow!("failed getting subscription_id"))?;

        let event: Event = serde_json::from_value(
            json.get(2)
                .ok_or_else(|| anyhow!("invalid JSON: {:?}", json))?
                .to_owned(),
        )?;

        Ok(Self::Event(event))
    }
}
