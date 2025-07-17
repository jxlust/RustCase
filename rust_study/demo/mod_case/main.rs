mod my_struct;
use crate::my_struct::MyStruct;
mod foo;
// use crate::foo::{Animal, MSG};

mod prelude {
    pub use crate::foo::{Animal, MSG};
}
use crate::prelude::*;

fn main() {
    let a: u32 = 1;
    let b: u32 = 2;
    let c: u32 = a.wrapping_sub(b);
    println!("{}", c); // 4294967295  == 2**32 - 1

    let a2: i32 = 1;
    let b2: i32 = 2;
    let c2 = a2 - b2;
    println!("{}", c2); // -1

    let _ms = MyStruct {};

    let animal = Animal::new();
    println!("{}", animal.name);
    animal.display_info();

    let _msg = MSG {
        msg: "hello".to_string(),
    };
}
