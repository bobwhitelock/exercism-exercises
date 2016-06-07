
pub fn score(string: &str) -> i32 {
    string
        .to_lowercase()
        .chars()
        .map(|c| letter_score(c))
        .collect::<Vec<_>>()
        .iter()
        .fold(0, |acc, i| acc + i)
}

fn letter_score(letter: char) -> i32 {
    match letter {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 0,
    }
}
