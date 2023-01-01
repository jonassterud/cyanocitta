use serde::Serialize;

/// Req.
#[derive(Debug, Serialize)]
pub struct Req {
    /// String representing a subscription.
    pub subscription_id: String,
    /// One or more filters.
    pub filters: Vec<Filter>,
}

/// Filter.
#[derive(Debug, Serialize)]
pub struct Filter {
    /// List of event ids or prefixes.
    pub ids: Vec<Vec<u8>>,
    /// List of pubkeys or prefixes, the pubkey of an event must be one of these.
    pub authors: Vec<Vec<u8>>,
    /// List of a kind numbers.
    pub kinds: Vec<u32>,
    /// List of event ids that are referenced in an "e" tag.
    #[serde(rename = "#e")]
    pub e: Vec<Vec<u8>>,
    /// List of pubkeys that are referenced in a "p" tag.
    #[serde(rename = "#p")]
    pub p: Vec<Vec<u8>>,
    /// UNIX timestamp, events must be newer than this to pass.
    pub since: i64,
    /// UNIX timestamp, events must be older than this to pass.
    pub until: i64,
    /// Maximum number of events to be returned in the initial query.
    pub limit: u32,
}

impl Req {
    /// Create [`Req`].
    /// 
    /// # Arguments
    /// 
    /// * `subscription_id` - string representing a subscription.
    /// * `filters` - list of one or more filters.
    pub fn new(subscription_id: String, filters: Vec<Filter>) -> Self {
        Self {
            subscription_id,
            filters,
        }
    }
}

impl Filter {
    /// Create [`Filter`].
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
        ids: Vec<Vec<u8>>,
        authors: Vec<Vec<u8>>,
        kinds: Vec<u32>,
        e: Vec<Vec<u8>>,
        p: Vec<Vec<u8>>,
        since: i64,
        until: i64,
        limit: u32,
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
