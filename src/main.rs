struct Person {
    name: String,
    age: u8
}

// rust default trait (available within Rust it self)
impl ToString for Person {

    fn to_string(&self) -> String {
        format!("My name is {} and I am {}.", self.name, self.age)
    }

}

// trait is like an interface in Java or similar language
fn main() {
    let adit = Person { name: String::from("Aditya Chandra"), age: 21 };

    println!("{}", adit.to_string()); // my name is Aditya Chandra and I am 21.
}