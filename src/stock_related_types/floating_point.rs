use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug, Clone)]

pub struct FloatingPoint {
    mantissa: i64,
    exponent: i8,
}

fn is_valid_numeric_with_optional_dot(s: &str) -> bool {
    let re = Regex::new(r"^\d*\.?\d*$").unwrap();
    re.is_match(s) && s != "." && !s.ends_with('.')
}

const PANIC_MESSAGE: &str = "Expected valid input: (12.23, 100)";

fn extract_float_from_valid_string(value: &str) -> FloatingPoint {
    if !is_valid_numeric_with_optional_dot(value) {
        panic!("{}, got: {}", PANIC_MESSAGE, value)
    }
    let value = value.trim();

    let parts: Vec<&str> = value.split('.').collect();

    match parts.len() {
        2 => {
            let mantissa = format!("{}{}", parts[0], parts[1]);
            FloatingPoint {
                mantissa: mantissa.parse::<i64>().unwrap(),
                exponent: parts[1].len() as i8 * -1,
            }
        }
        1 => FloatingPoint {
            mantissa: parts[0].parse::<i64>().unwrap(),
            exponent: 1,
        },
        _ => panic!("{}", PANIC_MESSAGE),
    }
}

impl FloatingPoint {
    pub fn new(mantissa: i64, exponent: i8) -> Self {
        FloatingPoint { mantissa, exponent }
    }

    pub fn construct_from_string(value_in_string: &str) -> Option<Self> {
        match value_in_string {
            "_" => None,
            "-" => None,
            value => Some(extract_float_from_valid_string(value)),
        }
    }
}

const TEN: u64 = 10;

fn ten_pow(num: i64, pow: i8) -> i64 {
    num * TEN.pow(pow as u32) as i64
}

fn are_equal_rhs_exponent_bigger(lhs: &FloatingPoint, rhs: &FloatingPoint) -> bool {
    let exp_diff = rhs.exponent - lhs.exponent;
    lhs.mantissa == ten_pow(rhs.mantissa, exp_diff)
}

impl PartialEq for FloatingPoint {
    fn eq(&self, other: &Self) -> bool {
        if self.exponent == other.exponent {
            return self.mantissa == other.mantissa;
        }

        if self.exponent < other.exponent {
            return are_equal_rhs_exponent_bigger(&self, &other);
        }

        are_equal_rhs_exponent_bigger(&other, &self)
    }
}

fn compare_exponents_equal(lhs: i64, rhs: i64) -> Option<Ordering> {
    if lhs < rhs {
        return Some(Ordering::Less);
    } else if lhs == rhs {
        return Some(Ordering::Equal);
    } else {
        return Some(Ordering::Greater);
    }
}

impl PartialOrd for FloatingPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.exponent == other.exponent {
            return compare_exponents_equal(self.mantissa, other.mantissa);
        } else if self.exponent < other.exponent {
            return compare_exponents_equal(
                self.mantissa,
                ten_pow(other.mantissa, other.exponent - self.exponent),
            );
        } else {
            return compare_exponents_equal(
                ten_pow(self.mantissa, self.exponent - other.exponent),
                other.mantissa,
            );
        }
    }
}
