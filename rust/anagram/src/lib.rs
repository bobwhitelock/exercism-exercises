
pub fn anagrams_for<'a>(word: &str, potentials: &[&'a str]) -> Vec<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort_str(&lowercase_word);

    let mut anagrams: Vec<&str> = vec![];
    for potential in potentials {
        let lowercase_potential = potential.to_lowercase();
        let sorted_potential = sort_str(&lowercase_potential) ;

        if lowercase_word != lowercase_potential &&
            sorted_word == sorted_potential {
            anagrams.push(potential);
        }
    }

    anagrams
}

fn sort_str(s: &str) -> String {
    let mut chars: Vec<_> = s.to_string().chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
