fn main() {
    // mutates a number by adding another number n
    let mut my_number: i32 = 15;
    let n: i32 = 5;
    sum_number(&mut my_number, &n);
    println!("{}", my_number);

    // concatenates two strings (also adds a space between them)
    let a_string: String = "Text A".to_string();
    let b_string: String = "Text B".to_string();
    let joined_string: String = join_strings(a_string, b_string);
    println!("{}", joined_string);

    // swaps the values of two variables
    let mut a: i32 = 31;
    let mut b: i32 = 20;
    println!("Before the swap the value of a: {a}\tb: {b}");
    swapping_values(&mut a, &mut b);
    println!("After the swap the value of a: {a}\tb: {b}");

    // returns a nicely formatted string congratulation for a given birthday
    let age: u8 = 20;
    let formatted_text: String = wish_happy_birthday(&age);
    println!("{formatted_text}");

    // repeats a string n times, thus producing a new string (advanced).
    let word = "Hello ";
    let mut n: i32 = 5;
    let mut text: String = "".to_string();
    repeat_word(&mut n, &word, &mut text);
    println!("{text}");

    // reverses the digits of a positive integer (advanced).
    // SOLUTION 1
    let mut digits: u32 = 12345;
    let reversed_digits = reverse_digits(&mut digits);
    println!("{}", reversed_digits);

    // SOLUTION 2
    let mut digits: u32 = 12345;
    let mut reversed_digits = 0;
    reverse_digits_2(&mut digits, &mut reversed_digits);
    println!("{}", reversed_digits);
}

// -------------------------------------------
// mutates a number by adding another number n
// -------------------------------------------
fn sum_number(base_num: &mut i32, n: &i32) {
    *base_num += *n;
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

// -------------------------------------------
// swaps the values of two variables
// -------------------------------------------
fn swapping_values(a: &mut i32, b: &mut i32) {
    let temporay_a: i32 = *a; // saving the value of A in a new temporary variable, so you can safely overwrite the original value of A with B
    *a = *b; // overwriting A's value with B's
    *b = temporay_a; // overwriting B's value with the temporary variable (the value of A)
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

// -------------------------------------------
// reverses the digits of a positive integer (advanced).
// SOLUTION 1
// -------------------------------------------
fn reverse_digits(digits: &mut u32) -> u32 { // it receives the digits that we will reverse
    let mut reversed_digits: u32 = 0; // here we will save the numbers one by one from the end

    reversing_digits(digits, &mut reversed_digits); // this is the interesting part

    reversed_digits // returns the reversed number
}

fn reversing_digits(digits: &mut u32, reversed_digits: &mut u32) {
    let length_of_digits = digits.to_string().len() as u32; // We convert the u32 to a string so we can get how long it is then we convert the lenght to u32

    if length_of_digits > 1 { // repeat while the lenght of the digits are more than 1
        /*
            what does the code below does?
            step 1: *digits
                we have the digits 12345
            step 2: *digits % 12345
                    we divide 12345 by 10 and with the sign % we get the remainder e.g. 5
                    / -> simple dividing
                    % -> with this you get the remainder
            step 3: *digits % 10 * u32::pow(10, length_of_digits - 1)
                    we multiply this e.g 5 with the 10^(5-1) -> 5 * 10000 = 50000
            step 4: we add this 50000 to the reversed_digits variable (which is 0 in the first run)
        */
        *reversed_digits += *digits % 10 * u32::pow(10, length_of_digits - 1);
        /*
            step 5: we divide the 12345 by 10, so we would get a float 1234.5, but we convert it into u32, so we get rid of the part after the . (the new value will be 1234)
        */
        *digits = *digits / 10 as u32;
        reversing_digits(digits, reversed_digits); // the function calls itself again
    } else {
        *reversed_digits += *digits; // here we know the digits variable only has one digit left so simply add this number to the reversed_digits
    }
}

// -------------------------------------------
// reverses the digits of a positive integer (advanced).
// SOLUTION 2
// -------------------------------------------
fn reverse_digits_2(digits: &mut u32, reversed_digits: &mut u32){
    let length_of_digits = digits.to_string().len() as u32;

    if length_of_digits > 1 {
        *reversed_digits += *digits % 10 * u32::pow(10, length_of_digits - 1);
        *digits = *digits / 10 as u32;
        reverse_digits_2(digits, reversed_digits);
    } else {
        *reversed_digits += *digits;
    }
}