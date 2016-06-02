
use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let punctuation = vec![
        '!', '"', '$', '%', '^', '&', '*', '(', ')', ':', ',', '.', '_', '-',
        '#', '@',
    ];

    let munged_input = input
        .chars()
        .map(|c| match c {
            _ if punctuation.contains(&c) => ' ',
            _ => c,
        })
        .collect::<String>();

    let mut word_counts = HashMap::new();

    let words = munged_input.split_whitespace();
    for word in words {
        let word = word.to_string().to_lowercase();
        let count = match word_counts.get(&word) {
            Some(current_count) => current_count + 1,
            None => 1,
        };
        word_counts.insert(word, count);
    }

    word_counts
}
