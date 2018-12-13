use std::fmt::{Display, Formatter, Result};

const UNITS_SYMBOLS: Symbols = Symbols {
    one: "I",
    five: "V",
    ten: "X",
};
const TENS_SYMBOLS: Symbols = Symbols {
    one: "X",
    five: "L",
    ten: "C",
};
const HUNDREDS_SYMBOLS: Symbols = Symbols {
    one: "C",
    five: "D",
    ten: "M",
};
const THOUSANDS_SYMBOLS: Symbols = Symbols {
    one: "M",
    five: "",
    ten: "",
};

struct Symbols<'a> {
    one: &'a str,
    five: &'a str,
    ten: &'a str,
}

pub struct Roman {
    units: usize,
    tens: usize,
    hundreds: usize,
    thousands: usize,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut formatted = String::new();
        formatted.push_str(&format_digit(self.thousands, THOUSANDS_SYMBOLS));
        formatted.push_str(&format_digit(self.hundreds, HUNDREDS_SYMBOLS));
        formatted.push_str(&format_digit(self.tens, TENS_SYMBOLS));
        formatted.push_str(&format_digit(self.units, UNITS_SYMBOLS));

        write!(f, "{}", formatted)
    }
}

fn format_digit<'a>(digit_value: usize, symbols: Symbols) -> String {
    let one_symbol = symbols.one;
    let five_symbol = symbols.five;
    let ten_symbol = symbols.ten;

    match digit_value {
        0 => "".to_string(),
        1...3 => one_symbol.repeat(digit_value),
        4 => (one_symbol.to_string() + five_symbol),
        5 => five_symbol.to_string(),
        6...8 => (five_symbol.to_string() + &one_symbol.repeat(digit_value - 5)),
        9 => (one_symbol.to_string() + ten_symbol),
        _ => panic!(format!("Got surprising digit value: {}", digit_value)),
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut digits = digits(num);

        Roman {
            units: pop_digit(&mut digits),
            tens: pop_digit(&mut digits),
            hundreds: pop_digit(&mut digits),
            thousands: pop_digit(&mut digits),
        }
    }
}

fn digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn pop_digit(digits: &mut Vec<u32>) -> usize {
    digits.pop().unwrap_or(0) as usize
}
