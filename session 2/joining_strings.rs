fn main() {
    // concatenates two strings (also adds a space between them)
    let a_string: String = "Text A".to_string();
    let b_string: String = "Text B".to_string();
    let joined_string: String = join_strings(a_string, b_string);
    println!("{}", joined_string);
}

// -------------------------------------------
// concatenates two strings (also adds a space between them)
// -------------------------------------------
fn join_strings(a_string: String, b_string: String) -> String {
    let mut joining_strings: String = a_string; // in this variable I save the first part of the string
    joining_strings.push_str(" "); // so there will be a space between the two texts
    joining_strings.push_str(&b_string); // adds the other text to the other
    joining_strings.to_string() // returns the joined strings
}
