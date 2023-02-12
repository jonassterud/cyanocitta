use super::ClientState;
use anyhow::{anyhow, Result};
use nostr_sdk::prelude::*;
use tokio::task::JoinHandle;

impl ClientState {
    pub async fn start_notifications_loop(&self) -> Result<JoinHandle<()>> {
        let inner = self.0.lock().await;
        let client = inner
            .client
            .as_ref()
            .ok_or_else(|| anyhow!("missing client"))?;

        let mut notifications_receiver = client.notifications();
        let handle = tokio::spawn(async move {
            loop {
                while let Ok(notification) = notifications_receiver.recv().await {
                    println!("{:?}", notification);
                    /*
                                       if let RelayPoolNotification::Event(_, event) = notification {
                                           match event.kind {
                                               Kind::Metadata => println!("todo.."),
                                               Kind::TextNote => {
                                                   println!("{:?}", event);
                                               },
                                               Kind::RecommendRelay => println!("todo.."),
                                               Kind::ContactList => println!("todo.."),
                                               Kind::EncryptedDirectMessage => println!("todo.."),
                                               Kind::EventDeletion => println!("todo.."),
                                               Kind::Repost => println!("todo.."),
                                               Kind::Reaction => println!("todo.."),
                                               Kind::ChannelCreation => println!("todo.."),
                                               Kind::ChannelMetadata => println!("todo.."),
                                               Kind::ChannelMessage => println!("todo.."),
                                               Kind::ChannelHideMessage => println!("todo.."),
                                               Kind::ChannelMuteUser => println!("todo.."),
                                               Kind::Authentication => println!("todo.."),
                                               Kind::Replaceable(_) => println!("todo.."),
                                               Kind::Ephemeral(_) => println!("todo.."),
                                               Kind::ParameterizedReplaceable(_) => println!("todo.."),
                                               Kind::Custom(_) => println!("todo.."),
                                           }
                                       } else {
                                           println!("todo..");
                                       }
                    */
                }
            }
        });

        Ok(handle)
    }
}
