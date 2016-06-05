
pub fn raindrops(number: i32) -> String {
    let pling = if number % 3 == 0 {"Pling"} else {""};
    let plang = if number % 5 == 0 {"Plang"} else {""};
    let plong = if number % 7 == 0 {"Plong"} else {""};
    let result = format!("{}{}{}", pling, plang, plong);
    if result.is_empty() {
        number.to_string()
    }
    else {
        result
    }
}
