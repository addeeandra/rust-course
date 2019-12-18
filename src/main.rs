fn main() {
    // let my_string = String::from("How's it going? My name is Adit.");
    let mut my_string = String::from("How's it going? My name is Adit.");

    println!("Length : {}", my_string.len());
    println!("Is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Does the string contains 'Adit'? {}", my_string.contains("Adit"));

    // my_string should let mut my_string
    my_string.push_str(" Welcome!");
    println!("{}", my_string);
}