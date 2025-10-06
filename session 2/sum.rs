fn main() {
    // mutates a number by adding another number n
    let mut my_number: i32 = 15;
    let n: i32 = 5;
    sum_number(&mut my_number, &n);
    println!("{}", my_number);
}

// -------------------------------------------
// mutates a number by adding another number n
// -------------------------------------------
fn sum_number(base_num: &mut i32, n: &i32) {
    *base_num += *n;
}