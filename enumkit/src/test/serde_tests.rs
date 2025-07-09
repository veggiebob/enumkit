use crate::{EnumMapping, EnumValues};
use serde::{Deserialize, Serialize};

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