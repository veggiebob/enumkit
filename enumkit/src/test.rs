use enumkit_derive::{EnumMapping, EnumValues};

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

}