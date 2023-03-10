use crate::types::{Event, Filter, SubscriptionId};
use serde::ser::{Serialize, Serializer};
use serde_json::json;

/// Message sent from client to relay.
///
/// https://github.com/nostr-protocol/nips#client-to-relay
#[derive(Clone, Debug)]
pub enum ClientMessage {
    Event { event: Event },
    Req { subscription_id: SubscriptionId, filters: Vec<Filter> },
    Close { subscription_id: SubscriptionId },
    Auth { signed_event: Event },
}

impl ClientMessage {
    /// Create [`ClientMessage::Auth`].
    pub fn new_auth(signed_event: Event) -> Self {
        Self::Auth { signed_event }
    }

    /// Create [`ClientMessage::Close`].
    pub fn new_close(subscription_id: SubscriptionId) -> Self {
        Self::Close { subscription_id }
    }

    /// Create [`ClientMessage::Event`].
    pub fn new_event(event: Event) -> Self {
        Self::Event { event }
    }

    /// Create [`ClientMessage::Req`].
    pub fn new_req(subscription_id: SubscriptionId, filters: Vec<Filter>) -> Self {
        Self::Req { subscription_id, filters }
    }
}

impl Serialize for ClientMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ClientMessage::Event { event } => ("EVENT", event).serialize(serializer),
            ClientMessage::Req { subscription_id, filters } => {
                let mut out = vec![json!("REQ"), json!(&subscription_id.0)];
                out.append(&mut filters.iter().map(|filter| json!(filter)).collect());
                out.serialize(serializer)
            }
            ClientMessage::Close { subscription_id } => ("CLOSE", &subscription_id.0).serialize(serializer),
            ClientMessage::Auth { signed_event } => ("AUTH", signed_event).serialize(serializer),
        }
    }
}
