use crate::{EnumMapping, EnumValues};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(PartialEq, Eq, Debug, EnumValues, EnumMapping)]
enum TestEnum {
    A,
    B,
    C
}

#[test]
fn len_test_1() {
    assert_eq!(TestEnum::len(), 3);
    assert_eq!(TestEnum::values().collect::<Vec<_>>(), vec![TestEnum::A, TestEnum::B, TestEnum::C]);
}

#[test]
fn mapping_test_1() {
    let map_1 = TestEnumMapping::new(|e| 1);
    for v in TestEnum::values() {
        assert_eq!(*map_1.get(v), 1);
    }
}

#[test]
fn test_serialize_deserialize_1() {
    let map_1 = TestEnumMapping::new(|e| 1);
    let serialized = serde_json::to_string(&map_1).unwrap();
    let deserialized: TestEnumMapping<i32> = serde_json::from_str(&serialized).unwrap();

    for v in TestEnum::values() {
        assert_eq!(*deserialized.get(v), 1);
    }
}



