## 模块化 module

1. super：上层模块 self：当前模块
2. 同一层级文件，利用 mod
3. 同一层级文件和文件夹下，必须要有入口文件 mod.rs;
4. path:

- 绝对路径从 crate 根开始，以 crate 名或者字面值 crate 开头
- 相对路径从当前模块开始，以 self、supper 或者当前模块的标识符开头。

5. 与main.rs 同一层级文件：use mydemo::{Log,Person}; mydemo是package name in Cargo.toml;


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



+ main.rs 代码
```rust

mod mylib;
// use self::mylib::log_info;
// // use self::mylib::log_info_dyn;
// use self::mylib::Animal;
// use self::mylib::City;
// use self::mylib::Person;
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