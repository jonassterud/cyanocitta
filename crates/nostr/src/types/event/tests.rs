use super::*;
use secp256k1::{rand, KeyPair, Secp256k1};

#[test]
pub fn create_event() {
    let secp = Secp256k1::new();
    let keys = KeyPair::new(&secp, &mut rand::thread_rng());
    let event = Event::new_signed(&keys, EventKind::ShortTextNote, vec![], "test".to_string()).unwrap();

    event.verify().unwrap();
}

#[test]
#[ignore = "todo"]
pub fn serialize_event() {
    todo!()
}

#[test]
#[ignore = "todo"]
pub fn deserialize_event() {
    todo!()
}
