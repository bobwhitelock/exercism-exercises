const MIN_SQUARE: u32 = 1;
const MAX_SQUARE: u32 = 64;

pub fn square(s: u32) -> u64 {
    if s < MIN_SQUARE || s > MAX_SQUARE {
        panic!("Square must be between {} and {}", MIN_SQUARE, MAX_SQUARE);
    }
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (MIN_SQUARE..(MAX_SQUARE + 1)).map(|s| square(s)).sum()
}
