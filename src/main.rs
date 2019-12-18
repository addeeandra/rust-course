fn main() {
    let n = 32;

    // n < 30, n > 30, n != 30
    if n > 30 {
        println!("The number is more than 30");
    } else if n < 30 {
        println!("The number is less than 30");
    } else {
        println!("The number is 30");
    }
}
