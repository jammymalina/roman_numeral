
#[derive(Debug, Clone, PartialEq)]
pub enum RomanNumeralError {
    InvalidStr,
    NumberOutOfRange,
}

#[derive(Debug, Clone)]
pub struct RomanNumeral {
    value: u32,
}

impl RomanNumeral {
    fn get_roman_symbols() -> Vec<char> {
        let roman_symbols = "MDCLXVI";
        roman_symbols.chars().collect()
    }

    fn get_symbol_value(symbol: char) -> Result<u32, RomanNumeralError> {
        let roman_symbols = Self::get_roman_symbols();
        let index = roman_symbols.into_iter().rev().position(|s| s == symbol);
        match index {
            Some(i) => Ok(5u32.pow(i as u32 % 2) * 10u32.pow(i as u32 / 2)),
            None => Err(RomanNumeralError::InvalidStr),
        }
    }

    pub fn max_value() -> u32 {
        let n = Self::get_roman_symbols().len() as u32;
        4u32.pow(n % 2) * 10u32.pow(n / 2) - 1
    }

    pub fn get(&self) -> String {
        let roman_symbols = Self::get_roman_symbols();
        let mut roman_numeral = vec![];
        let mut num = self.value;
        let mut symbol_index = roman_symbols.len() - 1;
        while num > 0 {
            let cipher = (num % 10) as usize;
            let rom_sym_1 = roman_symbols[symbol_index];
            let rom_sym_5 = roman_symbols[symbol_index.saturating_sub(1)];
            let rom_sym_10 = roman_symbols[symbol_index.saturating_sub(2)];
            let roman_symbol = match cipher {
                1..=3 => rom_sym_1.to_string().repeat(cipher),
                4 => vec![rom_sym_1, rom_sym_5].into_iter().collect(),
                5 => rom_sym_5.to_string(),
                6..=8 => std::iter::once(rom_sym_5)
                    .chain(std::iter::repeat(rom_sym_1).take(cipher - 5))
                    .collect(),
                9 => vec![rom_sym_1, rom_sym_10].into_iter().collect(),
                _ => String::from(""),
            };
            roman_numeral.push(roman_symbol);
            symbol_index = symbol_index.saturating_sub(2);
            num /= 10;
        }

        roman_numeral.into_iter().rev().collect::<Vec<String>>().join("")
    }

    pub fn get_u32(&self) -> u32 {
        self.value
    }

    pub fn from_u32(value: u32) -> Result<RomanNumeral, RomanNumeralError> {
        if value == 0 || value > Self::max_value() {
            return Err(RomanNumeralError::NumberOutOfRange);
        }

        Ok(RomanNumeral {
            value,
        })
    }

    pub fn from_string(value: &str) -> Result<RomanNumeral, RomanNumeralError> {
        let mut total = 0;
        let mut minus = 0;
        let symbols: Vec<_> = value.chars().collect();
        for i in 0..symbols.len() {
            let num = Self::get_symbol_value(symbols[i])?;
            if i == value.len() - 1 ||
                num >= Self::get_symbol_value(symbols[i + 1])?
            {
                total += num - minus;
                minus = 0;
                continue;
            }
            minus = num;
        }

        let result = RomanNumeral::from_u32(total);
        if result.is_err() {
            return Err(RomanNumeralError::InvalidStr);
        }
        let s = result.clone().unwrap().get();
        if s != value {
            return Err(RomanNumeralError::InvalidStr);
        }
        result
    }

}
