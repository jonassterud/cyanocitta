use serde::Serialize;

/// Close.
#[derive(Debug, Serialize)]
pub struct Close {
    /// String representing a subscription.
    pub subscription_id: String,
}

impl Close {
    /// Create [`Close`].
    ///
    /// # Arguments
    ///
    /// * `subscription_id` - string representing a subscription.
    pub fn new(subscription_id: String) -> Self {
        Self { subscription_id }
    }
}
