#[derive(PartialEq, Debug)]

pub struct FloatingPoint {
    mantissa: i64,
    exponent: i8,
}

fn extract_float_from_valid_string(value: &str) -> FloatingPoint {
    let value = value.trim();

    let parts: Vec<&str> = value.split('.').collect();

    match parts.len() {
        2 => {
            let mantissa = format!("{}{}", parts[0], parts[1]);
            FloatingPoint {
                mantissa: mantissa.parse::<i64>().unwrap(),
                exponent: parts[0].len() as i8 * -1,
            }
        }
        1 => FloatingPoint {
            mantissa: parts[0].parse::<i64>().unwrap(),
            exponent: 1,
        },
        _ => panic!("Expected valid input"),
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
