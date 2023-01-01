#[allow(unused_imports)]
use super::*;

#[test]
fn test_profile_new_with_random_keypair() {
    Profile::new_with_random_keypair();
}

#[test]
fn test_message_new_event() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let profile = Profile::new_with_random_keypair();
    Message::Event(
        message::Event::new(
            profile.public_key.serialize()[1..].to_vec(),
            SystemTime::duration_since(&SystemTime::now(), UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            1,
            vec![],
            "test".to_string(),
            profile.secret_key,
        )
        .unwrap(),
    );
}
