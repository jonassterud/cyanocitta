use super::*;
use serde_json::json;

#[test]
pub fn serialize_filter() {
    let filter = Filter::new()
        .ids(vec![EventId("event_id_1".to_string())])
        .kinds(vec![EventKind::ShortTextNote])
        .tags(FilterTags::new().e(vec![EventId("event_id_2".to_string())]))
        .since(0)
        .until(u32::MAX)
        .limit(5000);

    let serialized_filter = serde_json::to_string(&filter).unwrap();
    let json_filter = serde_json::to_string(&json!({
        "ids": ["event_id_1"],
        "kinds": [1],
        "#e": ["event_id_2"],
        "since": 0,
        "until": u32::MAX,
        "limit": 5000
    }))
    .unwrap();

    assert_eq!(serialized_filter, json_filter);
}
