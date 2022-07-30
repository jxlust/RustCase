mod mylib;
use self::mylib::log_info;
// use self::mylib::log_info_dyn;
use self::mylib::Animal;
use self::mylib::City;
use self::mylib::Person;
fn main() {
    let person = Person::new();
    let animal = Animal(String::from("pig"));
    let city = City(String::from("Beijing"));
    log_info(person);
    log_info(animal);
    //person animal has moved

    // log_info(city);//error
    // log_info_dyn(&person);
    // log_info_dyn(&animal);
}
