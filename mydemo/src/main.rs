struct Person {
    name: String, //fields
    last_name: String,
    age: u8,
}

impl Person {
    //associated funcition , as ask static function
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 1,
        }
    }
    fn from(name: String, last_name: String, age: u8) -> Person {
        Person {
            name,
            last_name,
            age,
        }
    }
    fn some_function() {
        println!("print some function..");
    }
    //instance method
    //第一个参数是self, 结构体的实例，在调用方法的时候
    //在impl里面, 类型Self是当前类型的别名
    //self: &Self === &self
    fn display_age(&self) {
        println!("Current Age: {}", self.age);
    }
    fn change_age(&mut self, age: u8) {
        self.age = age;
    }
}

fn main() {
    Person::some_function();
    // let mut person = Person {
    //     name: "xun liang".to_string(), //&str -> Strign
    //     last_name: "jiang".to_string(),
    //     age: 10,
    // };
    // let person_2 = Person {
    //     name: "xun liang".to_string(), //&str -> Strign
    //     last_name: "jiang".to_string(),
    //     age: 60,
    // };
    let mut person = Person::new();
    let person_2 = Person::from(String::from("Jxl"), String::from("ust"), 20);
    person.change_age(100);
    // person.display_age();
    // person_2.display_age();

    println!("{}-{}-{}", person.name, person.last_name, person.age);
    println!("{}-{}-{}", person_2.name, person_2.last_name, person_2.age);

}

fn box_use() {
    let num = 32; //in stack
    let num_2 = Box::new(100); //in heap by Box, use ptr(pointer)
    println!("{}", num_2);
}
