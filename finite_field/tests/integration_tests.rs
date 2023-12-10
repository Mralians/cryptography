use finite_field::field_element::*;
use num::traits::Pow;

const PRIME: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f"; // Prime used in Bitcoin's secp256k1

// Helper function
fn new_field_element(num: &str) -> FieldElement {
    FieldElement::new(num, PRIME).unwrap()
}

#[test]
fn test_addition() {
    let a = new_field_element("1a3b5c7d9e0f1a2b3c4d5e6f7a8b9c0d");
    let b = new_field_element("1234567890abcdef1234567890abcdef");
    let result = (a + b).unwrap();
    assert_eq!(
        result,
        new_field_element("2c6fb2f62ebae81a4e81b4e80b3769fc")
    );
}

#[test]
fn test_subtraction() {
    let a = new_field_element("deadbeef1234567890abcdef12345678");
    let b = new_field_element("1234567890abcdef1234567890abcdef");
    let result = (a - b).unwrap();
    assert_eq!(
        result,
        new_field_element("cc796876818888897e77777681888889")
    );
}

#[test]
fn test_multiplication() {
    let a = new_field_element("1234567890abcdef1234567890abcdeeaf");
    let b = new_field_element("fedcba9876543210fedcba9876543210ea");
    let result = (a * b).unwrap();
    assert_eq!(
        result,
        new_field_element("a00acd77d74247acc913f0513b7418fab2076f075787496d88fe5e4cf39a4445")
    );
}

#[test]
fn test_division() {
    let a = new_field_element("1234567890abcdef1234567890abcdef");
    let b = new_field_element("fedcba9876543210fedcba9876543210");
    let result = (a / b).unwrap();
    assert_eq!(
        result,
        new_field_element("cf9fcc1ecf87047d7006df225ee411f70a48a6eb425d6c6e5ab1aaee627e16c1")
    );
}

#[test]
fn test_pow() {
    let a = new_field_element("cf9fcc1ecf87047d7006df225ee411f70a48a6eb425d6c6e5ab1aaee627e16c1");
    let result = a.pow(234);
    assert_eq!(
        result,
        new_field_element("dbea5d4a522b629edd8e1570f93fdec743e2da13503c0107a7cb32add0bb4fc5")
    );
}
#[test]
#[should_panic(expected = "DifferentFields")]
fn test_different_fields() {
    let a = FieldElement::new("1", "13").unwrap();
    let b = FieldElement::new("1", PRIME).unwrap();
    let _ = (a + b).unwrap();
}
