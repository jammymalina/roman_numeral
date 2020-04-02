#[cfg(test)]
mod tests {
    use roman_numeral::{RomanNumeral, RomanNumeralError};

    fn test_numeral_error(numeral: Result<RomanNumeral, RomanNumeralError>,
        expected_error: RomanNumeralError)
    {
        match numeral {
            Err(e) => assert_eq!(e, expected_error),
            _ => panic!("It should have thrown an error"),
        };
    }

    #[test]
    fn it_should_return_max_value() {
        let max_value = RomanNumeral::max_value();
        assert_eq!(max_value, 3999);
    }

    #[test]
    fn it_should_create_from_roman_numeral_string() {
        let roman = RomanNumeral::from_string("I").unwrap();
        assert_eq!(roman.get_u32(), 1);

        let roman = RomanNumeral::from_string("III").unwrap();
        assert_eq!(roman.get_u32(), 3);

        let roman = RomanNumeral::from_string("X").unwrap();
        assert_eq!(roman.get_u32(), 10);

        let roman = RomanNumeral::from_string("CXXV").unwrap();
        assert_eq!(roman.get_u32(), 125);
    }

    #[test]
    fn it_should_not_create_from_roman_numeral_string() {
        let roman = RomanNumeral::from_string("IIII");
        test_numeral_error(roman, RomanNumeralError::InvalidStr);

        let roman = RomanNumeral::from_string("MMIIII");
        test_numeral_error(roman, RomanNumeralError::InvalidStr);

        let roman = RomanNumeral::from_string("MMXXXXIV");
        test_numeral_error(roman, RomanNumeralError::InvalidStr);
    }

    #[test]
    fn it_should_init_convert_basic_u32() {
        let roman = RomanNumeral::from_u32(1).unwrap();
        assert_eq!(roman.get(), String::from("I"));

        let roman = RomanNumeral::from_u32(4).unwrap();
        assert_eq!(roman.get(), String::from("IV"));

        let roman = RomanNumeral::from_u32(5).unwrap();
        assert_eq!(roman.get(), String::from("V"));

        let roman = RomanNumeral::from_u32(8).unwrap();
        assert_eq!(roman.get(), String::from("VIII"));

        let roman = RomanNumeral::from_u32(9).unwrap();
        assert_eq!(roman.get(), String::from("IX"));

        let roman = RomanNumeral::from_u32(10).unwrap();
        assert_eq!(roman.get(), String::from("X"));
    }

    #[test]
    fn it_should_convert_from_u32() {
        let roman = RomanNumeral::from_u32(49).unwrap();
        assert_eq!(roman.get(), String::from("XLIX"));

        let roman = RomanNumeral::from_u32(3999).unwrap();
        assert_eq!(roman.get(), String::from("MMMCMXCIX"));

        let roman = RomanNumeral::from_u32(86).unwrap();
        assert_eq!(roman.get(), String::from("LXXXVI"));
    }

    #[test]
    fn it_should_not_init_fromu32() {
        let roman = RomanNumeral::from_u32(0);
        test_numeral_error(roman, RomanNumeralError::NumberOutOfRange);

        let roman = RomanNumeral::from_u32(4000);
        test_numeral_error(roman, RomanNumeralError::NumberOutOfRange);
    }
}
