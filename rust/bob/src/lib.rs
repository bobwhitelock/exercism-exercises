
pub fn reply(input: &str) -> String {
    if input.is_empty() {
        "Fine. Be that way!"
    }
    else if is_uppercase(input) {
        "Whoa, chill out!"
    }
    else if is_question(input) {
        "Sure."
    }
    else {
        "Whatever."
    }.to_string()
}

fn is_uppercase(input: &str) -> bool {
    input == input.to_uppercase()
}

fn is_question(input: &str) -> bool {
    input.ends_with('?')
}
