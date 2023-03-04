use serde::{Deserialize, Serialize};

/// Nostr event content.
/// 
/// https://github.com/nostr-protocol/nips#event-kinds
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum EventContent {
    Metadata {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        about: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        picture: Option<String>
    },
    ShortTextNote(String),
    RecommendedRelay(String),
    Contacts(String),
    EncryptedDirectMessage(String),
    EventDeletion(String),
    Reaction(String),
    BadgeAward(String),
    ChannelCreation {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        about: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        picture: Option<String> },
    ChannelMetadata {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        about: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        picture: Option<String> },
    ChannelMessage(String),
    ChannelHideMessage {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>
    },
    ChannelMuteUser {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>
    },
    Reporting(String),
    ZapRequest(String),
    Zap(String),
    RelayListMetadata(String),
    ClientAuthentication(String),
    NostrConnect(String),
    ProfileBadges(String),
    BadgeDefinition(String),
    LongFormContent(String),
    ApplicationSpecificData(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_event_tag_serialization() {
        let pairs = vec![(
            EventContent::Metadata { name: Some("test".to_string()), about: Some("test".to_string()), picture: None },
            "[\"e\",\"event_id\",\"\",\"root\"]",
        )];

        for (event_content, serialized_event_content) in pairs {
            assert_eq!(serde_json::to_string(&event_content).unwrap(), serialized_event_content);
        }
    }
}
