#[cfg(test)]
use super::*;
use async_std::sync::{Arc, Mutex};

#[test]
fn create_profile() {
    Profile::new_with_random_keypair();
}

#[test]
fn create_message_event() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let profile = Profile::new_with_random_keypair();
    Message::Event(
        message::Event::new(
            profile.public_key,
            profile.secret_key,
            SystemTime::duration_since(&SystemTime::now(), UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            1,
            vec![],
            "test".to_string(),
        )
        .unwrap(),
    );
}

#[async_std::test]
#[ignore = "infinite loop"]
async fn create_client_and_get_nos() {
    use async_std::sync::{Arc, Mutex};

    let mut client = Client {
        app_data: Arc::new(Mutex::new(AppData::new_default_relays())),
        connections: vec![],
    };
    client.connect_to_relays().await.unwrap();

    client
        .send_message(Message::Req(message::Req::new(
            "25e5c82273a271cb1a840d0060391a0bf4965cafeb029d5ab55350b418953fbb".to_string(),
            Filters::default(),
        )))
        .await
        .unwrap();

    let buf: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(vec![]));

    let buf_1 = buf.clone();
    let sender_task = async_std::task::spawn(async {
        client
            .get_message(buf_1)
            .await
            .expect("failed getting message");
    });

    let buf_2 = buf.clone();
    let receiver_task = async_std::task::spawn(async move {
        loop {
            println!("{:?}", buf_2.lock().await);
            async_std::task::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    futures::join!(receiver_task, sender_task);
}

#[async_std::test]
#[ignore = "avoid spam"]
async fn relay_information_document() {
    let mut client = Client {
        app_data: Arc::new(Mutex::new(AppData::new_default_relays())),
        connections: vec![],
    };
    client.connect_to_relays().await.unwrap();
    println!("{:?}", client.app_data.lock().await.relays);
}
