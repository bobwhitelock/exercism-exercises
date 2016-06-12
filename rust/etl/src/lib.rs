
use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut result = BTreeMap::new();
    for (&points, letters) in input.iter() {
        for letter in letters.iter() {
            result.insert(letter.clone().to_lowercase(), points);
        }
    }
    result
}
