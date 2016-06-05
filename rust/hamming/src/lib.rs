
pub fn hamming_distance<'a>(strand: &'a str, other: &'a str) -> Result<u32, &'a str> {
    if strand.chars().count() != other.chars().count() {
        return Err("inputs of different length")
    }

    let distance = strand.chars().zip(other.chars())
        .filter(|&(s, o)| s != o)
        .count() as u32;
    Ok(distance)
}
