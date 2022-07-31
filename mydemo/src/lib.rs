pub mod test_mod {

    pub mod top_level {
        pub fn hi_there() {
            println!("hi there");
        }
        pub mod low_level {
            pub fn hello_world() {
                println!("hello world");
            }
        }
    }
}

pub trait Log {
    fn display_info(&self);
    // fn alert_message();//挂在全局
    fn alert_message(&self) {
        println!("Default alert message!");
    }
}

impl Log for Person {
    fn display_info(&self) {
        //absolute path
        //crate(大木箱)  points to -> src/main.rs
        // crate::test_mod::
        crate::test_mod::top_level::low_level::hello_world();

        //relative path
        test_mod::top_level::hi_there();

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
pub struct Animal(pub String);

pub struct City(pub String);

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
    pub fn name(&self) -> &String {
        &self.name
    }
}
