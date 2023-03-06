mod tags;

pub use tags::FilterTags;

use super::event::{EventId, EventKind};
use secp256k1::XOnlyPublicKey;
use serde::Serialize;

/// Nostr filter.
#[derive(Default, Serialize, Clone, Debug)]
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

impl Filter {
    /// Create [`Filter`].
    pub fn new() -> Filter {
        Self::default()
    }

    /// Set `ids` for filter.
    ///
    /// # Arguments
    /// * `ids` - event id must match one of these.
    pub fn ids(self, ids: Vec<EventId>) -> Self {
        Self { ids: Some(ids), ..self }
    }

    /// Set `authors` for filter.
    ///
    /// # Arguments
    /// * `authors` - event pubkey must match one of these.
    pub fn authors(self, authors: Vec<XOnlyPublicKey>) -> Self {
        Self { authors: Some(authors), ..self }
    }

    /// Set `kinds` for filter.
    ///
    /// # Arguments
    /// * `kinds` - event must match one of these kinds.
    pub fn kinds(self, kinds: Vec<EventKind>) -> Self {
        Self { kinds: Some(kinds), ..self }
    }

    /// Set `tags` for filter.
    ///
    /// # Arguments
    /// * `tags` - event must match at least one of each of these tags.
    pub fn tags(self, tags: FilterTags) -> Self {
        Self { tags: Some(tags), ..self }
    }

    /// Set `since` for filter.
    ///
    /// # Arguments
    /// * `since` - event must be newer than this to pass.
    pub fn since(self, since: u32) -> Self {
        Self { since: Some(since), ..self }
    }

    /// Set `until` for filter.
    ///
    /// # Arguments
    /// * `until` - event must be older than this to pass.
    pub fn until(self, until: u32) -> Self {
        Self { until: Some(until), ..self }
    }

    /// Set `limit` for filter.
    ///
    /// # Arguments
    /// * `limit` - maximum number of events to be returned.
    pub fn limit(self, limit: usize) -> Self {
        Self { limit: Some(limit), ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    pub fn serialize_filter() {
        let filter = Filter::new()
            .ids(vec![EventId("event_id_1".to_string())])
            .kinds(vec![EventKind::ShortTextNote])
            .tags(FilterTags::new().e(vec![EventId("event_id_2".to_string())]))
            .since(0)
            .until(u32::MAX)
            .limit(5000);

        let serialized_filter = serde_json::to_string(&filter).unwrap();
        let json_filter = serde_json::to_string(&json!({
            "ids": ["event_id_1"],
            "kinds": [1],
            "#e": ["event_id_2"],
            "since": 0,
            "until": u32::MAX,
            "limit": 5000
        }))
        .unwrap();

        assert_eq!(serialized_filter, json_filter);
    }
}
