fn main() {
    let mut n = 1;

    while n < 50 {

        // print only 5 multiplier
        if n % 5 == 0 {
            println!("n is {}", n);
        }

        n += 1;
    }
}
