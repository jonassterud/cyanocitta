use secp256k1::XOnlyPublicKey;
use serde::Serialize;

use crate::types::event::EventId;

/// Nostr filter tag.
///
/// https://github.com/nostr-protocol/nips/blob/master/12.md
#[derive(Serialize)]
pub struct FilterTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "#e")]
    pub e: Option<Vec<EventId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "#p")]
    pub p: Option<Vec<XOnlyPublicKey>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "#a")]
    pub a: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "#r")]
    pub r: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "#t")]
    pub t: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "#g")]
    pub g: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "#d")]
    pub d: Option<Vec<String>>,
}
