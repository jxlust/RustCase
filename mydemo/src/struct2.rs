//Struct no fields

struct AnimalTest(String, u32, String);
struct Animal(String);
fn main() {
    let animal = AnimalTest("dog".to_string(), 10, "bigdog".to_string());
    println!("{} {} {}", animal.0, animal.1, animal.2); //dog 10 bigdog

    let animal_2 = Animal(String::from("pig"));
    let Animal(animal_type) = animal_2;
    println!("{}", animal_type);
}
