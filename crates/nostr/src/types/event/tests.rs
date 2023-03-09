use super::*;
use secp256k1::{rand, KeyPair, Secp256k1};

#[test]
pub fn create_event() {
    let secp = Secp256k1::new();
    let keys = KeyPair::new(&secp, &mut rand::thread_rng());
    let event = Event::new_signed(&keys, EventKind::ShortTextNote, vec![], EventContent("test".to_string()))
        .unwrap()
        .verify()
        .unwrap();
    let event_serialized = serde_json::to_string(&event).unwrap();
    let event_deserialized = serde_json::from_str::<Event>(&event_serialized).unwrap();

    assert_eq!(event, event_deserialized);
}
