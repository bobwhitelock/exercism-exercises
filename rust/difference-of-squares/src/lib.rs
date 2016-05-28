
pub fn square_of_sum(num: u32) -> u32 {
    let sum = (0..num+1).fold(0, |total, x| total + x);
    sum.pow(2)
}

pub fn sum_of_squares(num: u32) -> u32 {
    let squares = (0..num+1).map(|x| x.pow(2));
    squares.fold(0, |total, x| total + x)
}

pub fn difference(num: u32) -> u32 {
    square_of_sum(num) - sum_of_squares(num)
}
