fn main() {
    // let numbers = [1, 2, 3, 4, 5];
    // let numbers: [type; size] = [...];
    // let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let numbers = [2; 10]; // 2 tenth times

    for n in numbers.iter() {
        println!("n is {}", n);
    }

    // access by index
    for n in 0..numbers.len() {
        println!("n2 is {}", numbers[n]);
    }

    println!("index 3 is {}", numbers[3]);
}