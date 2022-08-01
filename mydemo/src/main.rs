// mod lib;
// use lib::learning_rust::{Log, Person};
use mydemo::learning_rust::{Log, Person};
fn main() {
    let person = Person::new();
    // person.name;
    // person.display_info();
    println!("name:{}", person.name());
}
