## Rust 模块化

### 导入规则

#### 绝对导入

crate: absolute 绝对导入前缀,指向当前目的 root 模块(main.rs 或 lib.rs)。

#### 相对导入

- self:指向于当前模块的元素(类似./),用于任何想要引入自身模块所包含的内容时。(例如父模块重新导出子模块 :cake:)

```rust
pub use self:​:dog:​:Dog;
```

- super:指向父模块,用于从父模块导入元素(类似../)。

### 三种模块化方式

#### 1. 嵌套模块

```rust
mod animal{//声明模块
   pub struct Dog;
    pub struct Cat;
}
use animal::Dog;//使用模块中的内容
fn main() {
    let xxx = Dog;
}
```

#### 2. 文件模块

一个.rs 文件就是一个和文件同名的模块,在使用的地方要声明模块

```
project
  ++src
    --main.rs
    --animal.rs

```

```rust
//animal.rs
pub struct Dog;
```

```rust
//main.rs
mod animal;//声明模块
use animal::Dog;//使用模块内容
fn main() {
    let xxx = Dog;
}
```

#### 3. 目录模块

文件夹下同目录名和同文件名写法,比如 animal：

```
project
  ++src
    ++animal
        --dog.rs
        --cat.rs
    --animal.rs
    --main.rs
```

```rust
//dog.rs
pub struct Dog;
```

```rust
//cat.rs
pub struct Cat;
```

```rust
//animal.rs
pub mod dog;
pub mod cat;
use self::dog::Dog;
impl Dog {
    pub fn judge(){
        println!("This is a dog");
    }
}
```

```rust
//main.rs
mod animal;
use animal::dog::Dog;
fn main() {
    Dog::judge();
}
```

mod.rs 写法

```
project
  ++src
    ++animal
        --mod.rs
        --dog.rs
        --cat.rs
    --main.rs
```

```rust
//mod.rs
pub mod dog;
pub mod cat;
use self::dog::Dog;
impl Dog {
    pub fn judge(){
        println!("This is a dog");
    }
}
```

```rust
//main.rs
mod animal;
use animal::dog::Dog;
fn main() {
    Dog::judge();
}
```