
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz"
        .to_string()
        .chars()
        .collect::<HashSet<_>>();
    let sentence_chars = sentence.to_string()
        .to_lowercase()
        .chars()
        .filter(|c| alphabet.contains(c))
        .collect::<HashSet<_>>();
    let chars_not_in_sentence = alphabet.difference(&sentence_chars)
        .collect::<HashSet<_>>();
    chars_not_in_sentence.is_empty()
}
