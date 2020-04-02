# roman_numeral

Rust library for roman numerals. Encode/decode roman numerals with ease!

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
roman_numeral = "0.1"
```

**Example**

```rust
use roman_numeral::{RomanNumeral, RomanNumeralError};

// Create roman numeral
let roman = RomanNumeral::from_string("I").unwrap();
assert_eq!(roman.get_u32(), 1);

let roman = RomanNumeral::from_u32(49).unwrap();
assert_eq!(roman.get(), String::from("XLIX"));

// Get roman numeral (string)
let roman = RomanNumeral::from_u32(3999).unwrap();
let numeral = roman.get(); // MMMCMXCIX

// Get roman numeral decimal value
let roman = RomanNumeral::from_string("CXXV").unwrap();
let decimal_value = roman.get_u32(); // 125

// Get maximum possible value
let max_value = RomanNumeral::max_value();
assert_eq!(max_value, 3999);

// Invalid input
let error = RomanNumeral::from_string("IIII").unwrap_err();
assert_eq!(error, RomanNumeralError::InvalidStr);

let error = RomanNumeral::from_u32(0).unwrap_err();
assert_eq!(error, RomanNumeralError::NumberOutOfRange);

let error = RomanNumeral::from_u32(4000).unwrap_err();
assert_eq!(error, RomanNumeralError::NumberOutOfRange);
```


