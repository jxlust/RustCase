//trait narrowing
//trait 组合模式类似 提取方法执行
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
    let city = City(String::from("Beijing"));
    // log_info(person);
    // log_info(animal);
    //person animal has moved

    // log_info(city);//error
    log_info_dyn(&person);
    log_info_dyn(&animal);
}

//difference ?

//impl makes the compiler determine(决定) type at the compile time
//is will create multiple versions of the function, depending on
//how many types Log trait implements (Person,Animal...)

//use "val: &impl Log" is reference;
fn log_info(val: impl Log) {
    // val.alert_message();
    val.display_info();
}

//dyn is short for dynamic, and says that function should perform dynamic dispatch
//decission(判定) of exactly(精确的) which function to call at the runtime
fn log_info_dyn(val: &dyn Log) {
    val.display_info();
}
