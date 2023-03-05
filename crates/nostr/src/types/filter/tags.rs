use secp256k1::XOnlyPublicKey;
use serde::Serialize;

use crate::types::event::EventId;

/// Nostr filter tag.
///
/// https://github.com/nostr-protocol/nips/blob/master/12.md
#[derive(Default, Serialize)]
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

impl FilterTags {
    /// Create [`FilterTags`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `#e` for filter.
    pub fn e(self, e: Vec<EventId>) -> Self {
        Self { e: Some(e), ..self }
    }

    /// Set `#p` for filter.
    pub fn p(self, p: Vec<XOnlyPublicKey>) -> Self {
        Self { p: Some(p), ..self }
    }

    /// Set `#a` for filter.
    pub fn a(self, a: Vec<String>) -> Self {
        Self { a: Some(a), ..self }
    }

    /// Set `#r` for filter.
    pub fn r(self, r: Vec<String>) -> Self {
        Self { r: Some(r), ..self }
    }

    /// Set `#t` for filter.
    pub fn t(self, t: Vec<String>) -> Self {
        Self { t: Some(t), ..self }
    }

    /// Set `#g` for filter.
    pub fn g(self, g: Vec<String>) -> Self {
        Self { g: Some(g), ..self }
    }

    /// Set `#d` for filter.
    pub fn d(self, d: Vec<String>) -> Self {
        Self { d: Some(d), ..self }
    }
}
