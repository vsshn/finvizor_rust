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

#[test]
fn test_equality_normal() {
    assert_eq!(FloatingPoint::new(12230, -3), FloatingPoint::new(1223, -2));

    assert_eq!(FloatingPoint::new(-1, -3), FloatingPoint::new(-10, -4));
    assert_eq!(FloatingPoint::new(10, 0), FloatingPoint::new(10, 0));
    assert_eq!(FloatingPoint::new(10, 1), FloatingPoint::new(1000, -1));

    assert_ne!(FloatingPoint::new(-1, -3), FloatingPoint::new(1, -3));
    assert_ne!(FloatingPoint::new(10, 2), FloatingPoint::new(10, -2));
    assert_ne!(FloatingPoint::new(10, 0), FloatingPoint::new(10, 1));
}

#[test]
fn test_partial_order_same_exp() {
    assert!(FloatingPoint::new(2, 1) > FloatingPoint::new(1, 1));
    assert!(FloatingPoint::new(2, 0) < FloatingPoint::new(1, 1));
    assert!(FloatingPoint::new(13, 1) < FloatingPoint::new(14, 1));
    assert!(FloatingPoint::new(13, 1) > FloatingPoint::new(-13, 1));
    assert!(FloatingPoint::new(13, -1) > FloatingPoint::new(-13, -1));
}

#[test]
fn test_partial_order_diff_exp() {
    assert!(FloatingPoint::new(1, 2) > FloatingPoint::new(1, 1));
    assert!(FloatingPoint::new(13, -1) > FloatingPoint::new(130, -3));
    assert!(FloatingPoint::new(13, 2) > FloatingPoint::new(-13, 3));
    assert!(FloatingPoint::new(13, -1) > FloatingPoint::new(-13, -2));
}
