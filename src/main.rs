fn main() {
    let mut x = 10;

    {
        // shadowing
        let x = 15;
        println!("x is {}", x);
    }

    println!("x is {}", x);

    // shadowing
    let x = "X is a string";
    println!("x is {}", x);

    // shadowing
    let x = true;
    println!("x is {}", x);
}
