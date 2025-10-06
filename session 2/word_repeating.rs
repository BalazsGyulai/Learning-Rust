fn main() {
    // repeats a string n times, thus producing a new string (advanced).
    let word = "Hello ";
    let mut n: i32 = 5;
    let mut text: String = "".to_string();
    repeat_word(&mut n, &word, &mut text);
    println!("{text}");
}

// -------------------------------------------
// repeats a string n times, thus producing a new string (advanced).
// -------------------------------------------
fn repeat_word(n: &mut i32, word: &str, text: &mut String) { // n -> how many times, word -> the word that we will repeat, text -> we will write the words in this variable
    if *n > 0 {
        *n -= 1;
        text.push_str(&word);
        repeat_word(n, &word, text); // calling itself again until the n <= 0
    }
}