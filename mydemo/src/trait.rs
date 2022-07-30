//trait 特性/特点

trait Log {
    fn display_info(&self);
    // fn alert_message();//挂在全局
    fn alert_message(&self) {
        println!("Default alert message!");
    }
    fn new();
}

impl Log for Person {
    fn display_info(&self) {
        println!("display info:{} {} {} ", self.name, self.last_name, self.age);
    }
    fn new() {
        println!("New Person")
    }
}
impl Log for Animal {
    fn display_info(&self) {
        println!("display info:{}", self.0);
    }
    fn new() {
        println!("New Animal")
    }
}
struct Animal(String);

struct Person {
    name: String, //fields
    last_name: String,
    age: u32,
}
impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 9,
        }
    }
    fn from(name: String, last_name: String, age: u32) -> Person {
        Person {
            name,
            last_name,
            age,
        }
    }
}
fn main() {
    let person = Person::new();

    let animal = Animal(String::from("pig"));
    let Animal(animal_type) = &animal;
    println!("{}", animal_type);//pig

    person.display_info();//display info:Default Default 9
    animal.display_info();//display info:pig
    Animal::new();//New Animal
    
}
