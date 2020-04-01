
const ROMAN_NUMERAL_SYMBOLS: &str = "IVXLCDM";

#[derive(Debug)]
pub enum RomanNumeralError {
    InvalidStr,
    NumberOutOfRange,
}

#[derive(Debug)]
pub struct RomanNumeral {
    value: String,
}

impl RomanNumeral {
    pub fn is_valid_str(value: &str) -> bool {
        true
    }

    pub fn max_value() -> u32 {
        4999
    }

    pub fn get(&self) -> String {
        self.value.clone()
    }

    pub fn get_u32() -> u32 {
        0
    }

    pub fn from_string(value: &str) -> Result<RomanNumeral, RomanNumeralError> {
        if !Self::is_valid_str(value) {
            return Err(RomanNumeralError::InvalidStr);
        }

        Ok(RomanNumeral {
            value: String::from(value),
        })
    }

    pub fn from_u32(value: u32) -> Result<RomanNumeral, RomanNumeralError> {
        if value == 0 || value > Self::max_value() {
            return Err(RomanNumeralError::NumberOutOfRange);
        }

        Ok(RomanNumeral {
            value: String::from(""),
        })
    }
}
