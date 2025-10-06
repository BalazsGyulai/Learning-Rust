fn main() {
    // swaps the values of two variables
    let mut a: i32 = 31;
    let mut b: i32 = 20;
    println!("Before the swap the value of a: {a}\tb: {b}");
    swapping_values(&mut a, &mut b);
    println!("After the swap the value of a: {a}\tb: {b}");
}

// -------------------------------------------
// swaps the values of two variables
// -------------------------------------------
fn swapping_values(a: &mut i32, b: &mut i32) {
    let temporay_a: i32 = *a; // saving the value of A in a new temporary variable, so you can safely overwrite the original value of A with B
    *a = *b; // overwriting A's value with B's
    *b = temporay_a; // overwriting B's value with the temporary variable (the value of A)
}
