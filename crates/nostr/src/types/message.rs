//! Nostr message types.
//!
//! https://github.com/nostr-protocol/nips#message-types

use super::event::Event;
use super::filter::Filter;

/// Message sent from client to relay.
///
/// https://github.com/nostr-protocol/nips#client-to-relay
pub enum ClientMessage {
    Event { event: Event },
    Req { subscription_id: String, filters: Vec<Filter> },
    Close { subscription_id: String },
    Auth { signed_event: Event },
}

/// Message sent from relay to client.
///
/// https://github.com/nostr-protocol/nips#client-to-relay
pub enum RelayMessage {
    Event { subscription_id: String, event: Event },
    Notice { message: String },
    Eose { subscription_id: String },
    Ok { event_id: String, status: bool, message: String },
    Auth { challenge_string: String },
}
