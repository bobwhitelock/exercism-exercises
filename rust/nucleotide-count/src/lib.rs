
use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna_string: &str) -> usize {
    dna_string.chars()
        .filter(|c| *c == nucleotide)
        .count()
}

pub fn nucleotide_counts(dna_string: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for &nucleotide in NUCLEOTIDES.iter() {
        counts.insert(nucleotide, count(nucleotide, dna_string));
    }
    counts
}
