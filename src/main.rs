enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Heading up!"),
        Direction::Down => println!("Heading down!"),
        Direction::Left => println!("Heading left!"),
        Direction::Right => println!("Heading right!")
    }
}
