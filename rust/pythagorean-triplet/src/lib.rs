use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    // Much better algorithms exist, but this is fine for now ¯\_(ツ)_/¯.
    let max_c = sum / 2;
    for c in 1..max_c {
        for b in 1..c {
            let a = sum - c - b;
            if a > b {
                continue;
            }
            if a.pow(2) + b.pow(2) == c.pow(2) {
                triplets.insert([a, b, c]);
            }
        }
    }

    triplets
}
