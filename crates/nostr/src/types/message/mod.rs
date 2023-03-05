//! Nostr message types.
//!
//! https://github.com/nostr-protocol/nips#message-types

mod message_client;
mod message_relay;

pub use message_client::*;
pub use message_relay::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn serialize_client_message() {
        todo!()
    }

    #[test]
    pub fn deserialize_relay_message() {
        todo!()
    }
}
