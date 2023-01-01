mod close;
mod event;
mod req;

use anyhow::Result;
pub use close::*;
pub use event::*;
pub use req::*;

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
            Message::Event(event) => {
                let out = vec!["EVENT".to_owned(), serde_json::to_string(event)?];

                Ok(serde_json::to_string(&out)?)
            }
            Message::Req(req) => {
                let mut out = vec!["REQ".to_owned(), req.subscription_id.to_owned()];
                out.append(
                    &mut req
                        .filters
                        .iter()
                        .map(|x| Ok(serde_json::to_string(x)?))
                        .collect::<Result<Vec<String>>>()?,
                );

                Ok(serde_json::to_string(&out)?)
            }
            Message::Close(close) => {
                let out = vec!["CLOSE".to_owned(), close.subscription_id.to_owned()];

                Ok(serde_json::to_string(&out)?)
            }
        }
    }
}
