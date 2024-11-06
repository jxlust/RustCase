#[allow(dead_code)]
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("Now {:?} will print!", Structure(100));
    println!("Now deep {:#?}", Deep(Structure(99)));
    let person = Person {
        name: "jxl",
        age: 99,
    };
    println!("Now person {:#?}", person);
}
