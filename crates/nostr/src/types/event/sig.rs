use crate::types::Event;
use anyhow::{anyhow, Result};
use secp256k1::{
    hashes::{hex::FromHex, sha256},
    KeyPair, Secp256k1,
};
use serde::{Deserialize, Serialize};

/// Nostr event signature.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#events-and-signatures
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct EventSig(pub String);

impl EventSig {
    /// Generate [`EventSig`].
    pub fn generate(event: &Event, keys: &KeyPair) -> Result<Self> {
        let id = event.id.as_ref().ok_or_else(|| anyhow!("missing event id"))?;
        let hash = sha256::Hash::from_hex(&id.0)?.to_vec();
        let message = secp256k1::Message::from_slice(&hash)?;
        let sig = Secp256k1::new().sign_schnorr(&message, keys).to_string();

        Ok(Self(sig))
    }
}
