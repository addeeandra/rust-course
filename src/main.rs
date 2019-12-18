struct Color {
    red: u8, // 0 - 255
    green: u8,
    blue: u8
}

fn main() {
    let bg = Color { red: 255, green: 70, blue: 15 };
    let mut bg2 = Color { red: 255, green: 70, blue: 15 };

    // bg.blue = 25; // error
    bg2.blue = 25;

    println!("Bg Color is : {}, {}, {}", bg.red, bg.green, bg.blue);
    println!("Bg Color is : {}, {}, {}", bg2.red, bg2.green, bg2.blue);
}
