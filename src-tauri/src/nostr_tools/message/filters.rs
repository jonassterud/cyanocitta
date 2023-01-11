use serde::{Deserialize, Serialize};

/// Filters.
#[derive(Default, Deserialize, Serialize)]
pub struct Filters {
    /// List of event ids or prefixes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<Vec<u8>>>,
    /// List of pubkeys or prefixes, the pubkey of an event must be one of these.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Vec<u8>>>,
    /// List of a kind numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<u32>>,
    /// List of event ids that are referenced in an "e" tag.
    #[serde(rename = "#e")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<Vec<Vec<u8>>>,
    /// List of pubkeys that are referenced in a "p" tag.
    #[serde(rename = "#p")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<Vec<Vec<u8>>>,
    /// UNIX timestamp, events must be newer than this to pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    /// UNIX timestamp, events must be older than this to pass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    /// Maximum number of events to be returned in the initial query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}
