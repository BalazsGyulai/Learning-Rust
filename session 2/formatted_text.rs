fn main() {
    // returns a nicely formatted string congratulation for a given birthday
    let age: u8 = 20;
    let formatted_text: String = wish_happy_birthday(&age);
    println!("{formatted_text}");
}

// -------------------------------------------
// returns a nicely formatted string congratulation for a given birthday
// -------------------------------------------
fn wish_happy_birthday(age: &u8) -> String {
    let mut age_to_string: String = format!("{}", *age); // converting u8 to string
    age_to_string.push_str("th"); // adding th after the age

    let mut formatting_wish: String = Default::default(); // initializing an empty string
    formatting_wish.push_str(&format!("{word:*^30}\n", word = " HAPPY "));
    formatting_wish.push_str(&format!("{: ^30}\n", age_to_string));
    formatting_wish.push_str(&format!("{word:*^30}", word = " BIRTHDAY! "));

    formatting_wish.to_string() // returns the formatted text
}