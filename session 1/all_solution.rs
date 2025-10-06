fn main() {
    // calculates the absolute value of a number.
    println!("{}", absolute_val_of_a_number(-3));

    // enforces a number to be in a given range
    println!("{}", number_within_range(5));

    // determines the maximum of three numbers
    println!("{}", maximum(6, 5, 1));
    println!("{}", maximum(5, 8, 1));
    println!("{}", maximum(5, 6, 7));

    // determines the amount of days for a given month
    println!("{}", days_in_a_month(2, 1900));

    // prints the grade for this course based on four test results
    println!("{}", exam_result(100.0, 100.0, 100.0, 100.0));

    // prints the body mass index category for a given weight and height
    println!("{}", body_mass(70.0, 1.7));

    // calculates the factorial of a given number (advanced)
    let fact = factorial(5);
    println!("{}", fact);
}

// -------------------------------------
// calculates the absolute value of a number.
// -------------------------------------
fn absolute_val_of_a_number(val: i32) -> i32 {
    if val < 0 {
        // depending on the value it either returns the number itself or the number * -1
        val * -1
    } else {
        val
    }
}

// ------------------------------------
// enforces a number to be in a given range
// ------------------------------------
fn number_within_range(val: i32) -> i32 {
    if val < 0 { // in case the value is less than 0, it returns 0
        0
    } else if val > 10 {  // in case the value is more than 10, it returns 10
        10 
    } else { 
        val // in everyother case it returns the value itself
    }
}

// ------------------------------------
// determines the maximum of three numbers
// ------------------------------------
fn maximum(val1: i32, val2: i32, val3: i32) -> i32 {
    if val1 >= val2 && val1 >= val3 { // in case va1 >= val2 >= val3 it returns val1
        val1
    } else if val2 >= val1 && val2 >= val3 { // in case val2 >= val1 >= val3 it returns val2
        val2
    } else { // in every other we know val3 is the biggest so we just return val3
        val3
    }
}

// -----------------------------------
// determines the amount of days for a given month
// -----------------------------------
fn days_in_a_month(month: u8, year: u32) -> u8 {
    if
        month == 1 ||
        month == 3 ||
        month == 5 ||
        month == 7 ||
        month == 8 ||
        month == 10 ||
        month == 12
    { // when we know a month has 31 days we return 31
        31
    } else if month == 4 || month == 6 || month == 9 || month == 11 { // when we know a month has 30 days we return 30
        30
    } else { // otherwise the month is February, so we either return 28 or 29 or we calculate if it is a leap year or not from the year
        // basic stuff just return 28 or 29
        // 28
        // or
        // 29

        // calculate if the year is a leap year
        /*
            This is what you see in code below.
            To calculate if a year is a leap year: 
                - divide the year by 4; if the remainder is 0, it's a potential leap year. 
                - If the year also ends in "00" (a century year), it must then be divisible by 400 (with no remainder) to be a leap year. 
                - Otherwise, if the year is divisible by 4 but not by 100, it's a leap year. 
        */
        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0 { 
                    29 
                } else { 
                    28
                }
            } else {
                29
            }
        } else {
            28
        }
    }
}

// -------------------------------------
// prints the grade for this course based on four test results
// -------------------------------------
fn exam_result(test1: f32, test2: f32, test3: f32, test4: f32) -> u8 {
    let result: f32 = (test1 * 0.1 + test2 * 0.2 + test3 * 0.35 + test4 * 0.35).into();
    
    if result >= 90.0 {
        1
    } else if result >= 80.0 {
        2
    } else if result >= 70.0 {
        3
    } else if result >= 60.0 {
        4
    } else {
        5
    }
}

// ------------------------------------------
// prints the body mass index category for a given weight and height
// ------------------------------------------
// To calculate an adult's BMI category, use the formula: weight in kilograms / (height in meters x height in meters)
fn body_mass(weight: f32, height: f32) -> f32 { // weight in kilogramms, height in meters
    weight / (height * height)
}


// ------------------------------------------
// calculates the factorial of a given number (advanced)
// ------------------------------------------
fn factorial(num: i32) -> i32 {
    if num == 0 { // when the num reaches 0 the function won't call itself again
        1 // returns 1 so our calculation won't change, it prevents infinite recursion
        
    } else {
        let new_val: i32 = num * factorial(num - 1); // we create a new variable which multiples the current value by the factorial of the smaller number
        new_val
    }
}
