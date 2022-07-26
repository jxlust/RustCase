struct Person {
    name: String,
    last_name: String,
    age: u8,
}

fn main() {
    let person = Person{
        name: "xun liang".to_string(),//&str -> Strign
        last_name: "jiang".to_string(),
        age: 10
    };
    println!("{}-{}-{}",person.name,person.last_name,person.age)
}

fn box_use() {
    let num = 32; //in stack
    let num_2 = Box::new(100); //in heap by Box, use ptr(pointer)
    println!("{}", num_2);
}
