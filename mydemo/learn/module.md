## 模块化 module

1. super：上层模块 self：当前模块。super类似：../, self类似：./
2. 同一层级文件，利用 mod
3. 同一层级文件和文件夹下，必须要有入口文件 mod.rs;
4. path:

- 绝对路径从 crate 根开始，以 crate 名或者字面值 crate 开头
- 相对路径从当前模块开始，以 self、supper 或者当前模块的标识符开头。

5. 与 main.rs 同一层级文件：use mydemo::{Log,Person}; mydemo 是 package name in Cargo.toml;


## 示例

- src/main.rs

```rust
mod mylib;
use self::mylib::Animal;
```

- src/mylib/mod.rs

```rust
//import the msg.rs
pub mod msg;
use msg::MESSAGE;

pub struct Animal(pub String);
```

- src/mylib/msg.rs

```rust
pub const MESSAGE: &str = "STATIC MESSAGE";
```

## mod 用法

- 定义 mod

```rust
pub mod test_mod{
    pub fn test_print(){
        println!("test print")
    }
}
//use
use xxx::test_mod;
```

- 导入同级文件名字直接

```rust
pub mod msg;//file: ./msg.rs
use msg::MESSAGE;
```

- 多层嵌套

```rust
pub mod test_mod {
    pub fn test_print() {
        println!("test print")
    }

    mod top_level {
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
```

```rust
// use:....
//absolute path
//crate(大木箱)  points to -> src/main.rs
crate::mylib::test_mod::top_level::low_level::hello_world();
//relative path
test_mod::top_level::hi_there();

```

## 其他

- 导入可以用\*一次性全部倒入模块公开的数据和方法

```rust
use self::mylib::*;
```

- 利用{}大括号导入

```rust
use self::mylib::{Animal,xxx_fun,...};
```

- std rust 标准库,比如

```rust
use std::array;
```

- main.rs 代码

```rust

// mod lib;
// use lib::{Log, Person};
use mydemo::learning_rust::{Log, Person};
fn main() {
    let person = Person::new();
    // person.name;
    // person.display_info();
    println!("name:{}", person.name());
}


mod mylib;
use self::mylib::{Log, Person};
// use self::mylib::test_mod;
fn main() {
    let person = Person::new();
    person.display_info();
    //person animal has moved

    // log_info(city);//error
    // log_info_dyn(&person);
    // log_info_dyn(&animal);
}


```

lib.rs

```rust
fn outsider() {
    println!("outsider fn!");
}
pub mod learning_rust {
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
            crate::learning_rust::top_level::low_level::hello_world();

            //relative path
            top_level::hi_there();

            //
            super::outsider();


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
}

```
