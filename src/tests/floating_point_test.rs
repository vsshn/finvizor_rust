use crate::stock_related_types::floating_point::FloatingPoint;

#[test]
fn test_conversion_normal() {
    assert_eq!(
        FloatingPoint::construct_from_string("12.23"),
        Some(FloatingPoint::new(1223, -2))
    );
}
