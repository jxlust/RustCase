pub mod msg;
use msg::MESSAGE;
trait Log {
    fn display_info(&self);
    // fn alert_message();//挂在全局
    fn alert_message(&self) {
        println!("Default alert message!");
    }
}

impl Log for Person {
    fn display_info(&self) {
        println!(
            "display info:{} {} {} ",
            self.name, self.last_name, self.age
        );
    }
}
impl Log for Animal {
    fn display_info(&self) {
        println!("display info:{}", self.0);
    }
}
struct Animal(String);

struct City(String);

pub struct Person {
    name: String, //fields
    last_name: String,
    age: u32,
}
impl Person {
    pub fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 9,
        }
    }
    pub fn from(name: String, last_name: String, age: u32) -> Person {
        Person {
            name,
            last_name,
            age,
        }
    }
}

//use "val: &impl Log" is reference;
fn log_info(val: impl Log) {
    // val.alert_message();
    val.display_info();
}

fn log_info_dyn(val: &dyn Log) {
    val.display_info();
}
