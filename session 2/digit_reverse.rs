fn main() {
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