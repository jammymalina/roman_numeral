#[cfg(test)]
mod tests {
    use roman_numeral::RomanNumeral;
    
    #[test]
    fn it_should_convert_from_u32() {
        let roman_1 = RomanNumeral::from_u32(1).unwrap();
        assert_eq!(roman_1.get(), String::from("I"));
    }
}
