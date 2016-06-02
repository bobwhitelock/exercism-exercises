
use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let munged_input = input
        .chars()
        .map(|c| match c {
            _ if c.is_alphanumeric() => c,
            _ => ' ',
        })
        .collect::<String>()
        .to_lowercase();

    let mut word_counts = HashMap::new();

    let words = munged_input.split_whitespace();
    for word in words {
        let word = word.to_string();
        let count = match word_counts.get(&word) {
            Some(current_count) => current_count + 1,
            None => 1,
        };
        word_counts.insert(word, count);
    }

    word_counts
}
