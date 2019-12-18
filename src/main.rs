fn main() {
    let tup1 = (20, 25, 30, 35);
    let tup2 = (10, 3.4, "Rust", false, (1, 4, 6));

    let tup3 = (45, 6.7, "Computer");
    let (a, b, c) = tup3;

    println!("{}", tup1.0);
    println!("{}", tup2.2);
    println!("{}", (tup2.4).2);
    println!("a is {}, b is {}, c is {}", a, b, c);
}