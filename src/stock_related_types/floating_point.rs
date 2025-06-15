use regex::Regex;

#[derive(PartialEq, Debug)]

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
        panic!("{}", PANIC_MESSAGE)
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
        FloatingPoint {
            mantissa: mantissa,
            exponent: exponent,
        }
    }

    pub fn construct_from_string(value_in_string: &str) -> Option<Self> {
        match value_in_string {
            "_" => None,
            value => Some(extract_float_from_valid_string(value)),
        }
    }
}
