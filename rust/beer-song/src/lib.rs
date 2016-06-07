
pub fn verse(number: i32) -> String {
    format!(
        "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
        capitalize_first(bottles_text(number).as_ref()),
        bottles_text(number),
        next_action_text(number),
        bottles_text(number - 1))
}

fn bottles_text(number: i32) -> String {
    match number {
        -1 => bottles_text(99),
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", number),
    }
}

fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars().collect::<Vec<_>>();
    chars[0] = chars[0].to_uppercase().next().unwrap();
    chars.into_iter().collect()
}

fn next_action_text(number: i32) -> String {
    match number {
        0 => "Go to the store and buy some more".to_string(),
        _ => format!("Take {} down and pass it around", article_text(number)),
    }
}

fn article_text(number: i32) -> String {
    match number {
        1 => "it",
        _ => "one",
    }.to_string()
}

pub fn sing(from_verse_number: i32, to_verse_number: i32) -> String {
    (to_verse_number..from_verse_number + 1)
        .rev()
        .map(|verse_number| verse(verse_number))
        .collect::<Vec<_>>()
        .join("\n")
}
