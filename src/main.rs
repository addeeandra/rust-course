fn main() {
    let mut x = 10;
    // let xr = &x; // references to x

    println!("x is {}", x);

    { // only able to borrow 1 mutable references (safety call inside a block)
        let dom = &mut x; // references to x (mutable)
        *dom += 1;
        println!("dom is {}", dom);
    }

    println!("x is {}", x);
}
