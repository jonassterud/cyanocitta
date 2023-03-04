use anyhow::{anyhow, Result};
use serde::{
    de::{self, Deserialize, Visitor},
    ser::{Serialize, Serializer},
};

/// Nostr event kinds.
///
/// https://github.com/nostr-protocol/nips#event-kinds
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum EventKind {
    Metadata = 0,
    ShortTextNote = 1,
    RecommendedRelay = 2,
    Contacts = 3,
    EncryptedDirectMessage = 4,
    EventDeletion = 5,
    Reaction = 7,
    BadgeAward = 8,
    ChannelCreation = 40,
    ChannelMetadata = 41,
    ChannelMessage = 42,
    ChannelHideMessage = 43,
    ChannelMuteUser = 44,
    //PublicChatReserved = 44..49,
    Reporting = 1984,
    ZapRequest = 9734,
    Zap = 9735,
    RelayListMetadata = 10002,
    ClientAuthentication = 22242,
    NostrConnect = 24133,
    ProfileBadges = 30008,
    BadgeDefinition = 30009,
    LongFormContent = 30023,
    ApplicationSpecificData = 30078,
    //RegularEvents = 1000..9999,
    //ReplaceableEvents = 10000..19999,
    //EphemeralEvents = 20000..29999,
    //ParameterizedReplaceableEvents = 30000..39999
}

struct EventKindVisitor;

impl<'de> Visitor<'de> for EventKindVisitor {
    type Value = EventKind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A JSON sequence")
    }

    fn visit_u32<E>(self, v: u32) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::try_from(v).map_err(|err| de::Error::custom(err))
    }
}

impl<'de> Deserialize<'de> for EventKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(EventKindVisitor)
    }
}

impl Serialize for EventKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

impl TryFrom<u32> for EventKind {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Metadata),
            1 => Ok(Self::ShortTextNote),
            2 => Ok(Self::RecommendedRelay),
            3 => Ok(Self::Contacts),
            4 => Ok(Self::EncryptedDirectMessage),
            5 => Ok(Self::EventDeletion),
            7 => Ok(Self::Reaction),
            8 => Ok(Self::BadgeAward),
            40 => Ok(Self::ChannelCreation),
            41 => Ok(Self::ChannelMetadata),
            42 => Ok(Self::ChannelMessage),
            43 => Ok(Self::ChannelHideMessage),
            44 => Ok(Self::ChannelMuteUser),
            1984 => Ok(Self::Reporting), // literally 1984
            9734 => Ok(Self::ZapRequest),
            9735 => Ok(Self::Zap),
            10002 => Ok(Self::RelayListMetadata),
            22242 => Ok(Self::ClientAuthentication),
            24133 => Ok(Self::NostrConnect),
            30008 => Ok(Self::ProfileBadges),
            30009 => Ok(Self::BadgeDefinition),
            30023 => Ok(Self::LongFormContent),
            30078 => Ok(Self::ApplicationSpecificData),
            _ => Err(anyhow!("Unknown event kind")),
        }
    }
}
