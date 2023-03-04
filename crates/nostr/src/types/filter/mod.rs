mod tags;

pub use tags::FilterTags;

use super::event::{EventId, EventKind};
use secp256k1::XOnlyPublicKey;
use serde::Serialize;

#[derive(Serialize)]
pub struct Filter {
    /// List of event ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<EventId>>,
    /// List of pubkeys (the pubkey of an event must be one of these).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<XOnlyPublicKey>>,
    /// List of a kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<EventKind>>,
    /// Generic tag queries (https://github.com/nostr-protocol/nips/blob/master/12.md).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub tags: Option<FilterTags>,
    /// UNIX timestamp (events must be newer than this to pass).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u32>,
    /// UNIX timestamp (events must be older than this to pass).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<u32>,
    /// Maximum number of events to be returned in the initial query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_filter_serialization() {
        let pairs = vec![(
            Filter {
                ids: None,
                authors: None,
                kinds: Some(vec![EventKind::ShortTextNote]),
                tags: Some(FilterTags {
                    e: Some(vec!["event_id_0".to_string(), "event_id_1".to_string()]),
                    p: None,
                    a: None,
                    r: None,
                    t: Some(vec!["hashtag_0".to_string(), "hashtag_1".to_string()]),
                    g: None,
                    d: None,
                }),
                since: None,
                until: None,
                limit: Some(500),
            },
            "{\"kinds\":[1],\"#e\":[\"event_id_0\",\"event_id_1\"],\"#t\":[\"hashtag_0\",\"hashtag_1\"],\"limit\":500}",
        )];

        for (filter, serialized_filter) in pairs {
            assert_eq!(serde_json::to_string(&filter).unwrap(), serialized_filter);
        }
    }
}
