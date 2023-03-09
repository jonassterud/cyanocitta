use secp256k1::{
    hashes::{sha256, Hash},
    rand::{
        self,
        distributions::{Distribution, Uniform},
    },
};

/// Subscription id used to represent a subcription.
///
/// https://github.com/nostr-protocol/nips/blob/master/01.md#from-client-to-relay-sending-events-and-creating-subscriptions
#[derive(Clone, Debug)]
pub struct SubscriptionId(pub String);

impl SubscriptionId {
    /// Create [`SubscriptionId`] hash based on `name`.
    pub fn from_name(name: &str) -> Self {
        let data = format!("{name}_cyanocitta"); // does this need to be more absolutely unique?
        let hash = sha256::Hash::hash(data.as_bytes()).to_string();

        Self(hash)
    }

    /// Create random [`SubscriptionId`].
    pub fn new() -> Self {
        // Stupid?
        let between = Uniform::from(b'a'..=b'z');
        let mut rng = rand::thread_rng();
        let id = (0..64).map(|_| between.sample(&mut rng) as char).collect::<String>();

        Self(id)
    }
}
