fn main() {
    let x = 10;
    {
        // isolated
        let y = 5;

        println!("x: {}, y: {}", x, y);
    }

    println!("x: {}, y: {}", x, y); // error can't find y
}
