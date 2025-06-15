use crate::stock_related_types::floating_point::FloatingPoint;

#[test]
fn test_conversion_normal() {
    assert_eq!(
        FloatingPoint::construct_from_string("12.23"),
        Some(FloatingPoint::new(1223, -2))
    );

    assert_eq!(
        FloatingPoint::construct_from_string("1223"),
        Some(FloatingPoint::new(1223, 1))
    );

    assert_eq!(
        FloatingPoint::construct_from_string("1223.0"),
        Some(FloatingPoint::new(12230, -1))
    );

    assert_eq!(FloatingPoint::construct_from_string("_"), None);
}

#[test]
#[should_panic(expected = "Expected valid input: (12.23, 100)")]
fn test_should_panic_if_non_numeric_or_dot() {
    FloatingPoint::construct_from_string("abc");
}

#[test]
#[should_panic(expected = "Expected valid input: (12.23, 100)")]
fn test_should_panic_if_too_many_dots() {
    FloatingPoint::construct_from_string("123.123.123");
}

#[test]
#[should_panic(expected = "Expected valid input: (12.23, 100)")]
fn test_should_panic_ends_with_dot() {
    FloatingPoint::construct_from_string("1223.");
}
