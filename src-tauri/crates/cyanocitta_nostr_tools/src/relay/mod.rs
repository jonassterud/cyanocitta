use anyhow::Result;
use serde::Deserialize;

/// Relay.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Relay {
    /// Websocket address.
    pub id: String,
    /// String identifying relay.
    pub name: String,
    /// String with detailed information.
    pub description: String,
    /// Administrative contact pubkey.
    pub pubkey: String,
    /// Administrative alternate contact.
    pub contact: String,
    /// A list of NIP numbers supported by the relay.
    pub supported_nips: Vec<u32>,
    /// String identifying relay software URL.
    pub software: String,
    /// String version identifier.
    pub version: String,
}

impl Relay {
    /// Create [`Relay`] from "Relay Information Document". See [NIP-11](https://github.com/nostr-protocol/nips/blob/master/11.md).
    ///
    /// # Arguments
    ///
    /// * `url` - websocket URL.
    pub fn new(websocket_url: &str) -> Result<Self> {
        let mut relay: Self = serde_json::from_reader(
            ureq::get(&websocket_url.replace("wss", "http"))
                .set("Accept", "application/nostr+json")
                .call()?
                .into_reader(),
        )?;

        relay.id = websocket_url.to_owned();

        Ok(relay)
    }
}
