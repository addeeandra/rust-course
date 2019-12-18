struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let blue = Color { red: 0, green: 0, blue: 255 };
    let red = Color { red: 255, green: 0, blue: 0 };

    print_color_1(blue);
    print_color_1(blue); // moved value, gain error

    print_color_2(&red);
    print_color_2(&red); // not moved, cause the references are passed instead
}

fn print_color_1(c: Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}

// pass by references to avoid borrowing
fn print_color_2(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}