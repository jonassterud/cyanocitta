use serde::{Deserialize, Serialize};

/// Req.
#[derive(Debug, Serialize)]
pub struct Req {
    /// String representing a subscription.
    pub subscription_id: String,
    /// A filter.
    pub filters: Filters,
}

/// Filter.
#[derive(Debug, Default, Deserialize, Serialize)]
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

impl Req {
    /// Create [`Req`].
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - string representing a subscription.
    /// * `filters` - [`Filters`].
    pub fn new(subscription_id: String, filters: Filters) -> Self {
        Self {
            subscription_id,
            filters,
        }
    }
}

impl Filters {
    /// Create [`Filters`].
    ///
    /// # Arguments
    ///
    /// * `ids` - list of event ids or prefixes.
    /// * `authors` - list of pubkeys or prefixes, the pubkey of an event must be one of these.
    /// * `kinds` - list of a kind numbers.
    /// * `e` - list of event ids that are referenced in an "e" tag.
    /// * `p` - list of pubkeys that are referenced in a "p" tag.
    /// * `since` - UNIX timestamp, events must be newer than this to pass.
    /// * `until` - UNIX timestamp, events must be older than this to pass.
    /// * `limit` - maximum number of events to be returned in the initial query.
    pub fn new(
        ids: Option<Vec<Vec<u8>>>,
        authors: Option<Vec<Vec<u8>>>,
        kinds: Option<Vec<u32>>,
        e: Option<Vec<Vec<u8>>>,
        p: Option<Vec<Vec<u8>>>,
        since: Option<i64>,
        until: Option<i64>,
        limit: Option<u32>,
    ) -> Self {
        Self {
            ids,
            authors,
            kinds,
            e,
            p,
            since,
            until,
            limit,
        }
    }
}
