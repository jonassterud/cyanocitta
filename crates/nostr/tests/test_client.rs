use std::str::FromStr;

use nostr::prelude::*;
use secp256k1::XOnlyPublicKey;

#[tokio::test]
#[ignore = "infinite loop"]
async fn test_client() {
    let mut client = Client::new();

    let mut relay = Relay::new("wss://nos.lol".into());
    relay.listen(100).await.unwrap();

    client.add_relay(relay);
    let mut receiver = client.listen(100).await.unwrap();

    client
        .send_message(
            "wss://nos.lol".to_string(),
            ClientMessage::new_req(
                SubscriptionId::new(),
                vec![Filter::new().limit(100).authors(vec![XOnlyPublicKey::from_str(
                    "32e1827635450ebb3c5a7d12c1f8e7b2b514439ac10a67eef3d9fd9c5c68e245",
                )
                .unwrap()])],
            ),
        )
        .await
        .unwrap();

    while let Ok(message) = receiver.recv().await {
        println!("message: {:?}", message);
    }
}
