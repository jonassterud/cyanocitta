use super::*;

impl Client {
    /// The start function does the following:
    /// * Connect to relays.
    /// * Listen for messages from relays (async loop).
    pub fn start(client: Arc<Mutex<Self>>) {
        tokio::spawn(async move {
            // ...
        });
    }
}
