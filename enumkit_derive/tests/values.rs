use enumkit::EnumValues;

#[derive(PartialEq, Eq, Debug, EnumValues)]
pub enum TestEnum {
    A,
    B
}

#[test]
pub fn test_values_1() {
    let x: Vec<_> = TestEnum::values().collect();
    assert_eq!(x, vec![TestEnum::A, TestEnum::B]);
}