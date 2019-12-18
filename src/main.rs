fn main() {
    let numbers = 30..36;
    for i in numbers {
        println!("The number is {}", i);
    }

    let animals = vec!["Rabbit", "Dog", "Cat"];
    for a in animals.iter() {
        println!("The animal name is {}", a);
    }

    let people = vec!["Andi", "Rudi", "Adit"];
    for (index, p) in people.iter().enumerate() {
        println!("Person number {} is {}", index, p);
    }
}
