mod msg;
use msg::MESSAGE;
pub use msg::MSG;
pub struct Animal {
    pub name: String,
    age: i32,
}

impl Animal {
    pub fn new() -> Animal {
        Animal {
            name: MESSAGE.to_owned(),
            age: 99,
        }
    }

    pub fn display_info(&self) {
        println!("display_info: name:{},age:{}", self.name, self.age);
    }
    fn change_age(&mut self, age: i32) {
        self.age = age;
    }
}
