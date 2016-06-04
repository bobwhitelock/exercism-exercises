
pub fn hamming_distance<'a>(strand: &'a str, other: &'a str) -> Result<u32, &'a str> {
    if strand.chars().count() != other.chars().count() {
        return Err("inputs of different length")
    }

    let distance = strand.chars().zip(other.chars())
        .fold(0, |acc, (s, o)| if s != o {acc + 1} else {acc + 0});
    Ok(distance)
}
