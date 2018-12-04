pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let digits_length = digits.len() as u32;
    let summed_digit_powers: u32 = digits.iter().map(|d| d.pow(digits_length)).sum();
    summed_digit_powers == num
}

fn digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
