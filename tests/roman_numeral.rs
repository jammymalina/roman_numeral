#[cfg(test)]
mod tests {
    use roman_numeral::RomanNumeral;

    #[test]
    fn it_should_return_max_value() {
        let max_value = RomanNumeral::max_value();
        assert_eq!(max_value, 3999);
    }

    #[test]
    fn it_should_convert_from_basic_u32() {
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
}
