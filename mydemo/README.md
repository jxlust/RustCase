## 包管理 cargo

```
cargo init
cargo run
```

## 机器码文件

```shell
#打印编译main.rs文件:
xxd -g1 main
```

## rust 系统三大核心
1. 借用
2. 所有权
3. 生命周期

## 学习目录learn
+ number.rs
+ stringAnd&str.rs
+ stackHeap.rs
+ funcTest.rs
+ valueMove.rs
+ reference.rs dereference.rs
+ struct.rs struct2.rs 
+ enums.rs
+ trait.rs trait2.rs
+ module.md 

## 整数除法

- usize 取用的都是向下取整
- 负数 是向上。
- 综合来说：貌似是向零取整

```js
 11 / 4 = 2
 -7 / 8 = 0
```

- 无符号整数溢出问题

```rust
let a:usize = 1;
// "a - 9" is error overflow
println!("{}",(a.wrapping_sub(9)));// 1 - 9
```

- 负数对正数取模

```rust
// (-7) % 8
println!("{}",(-7) % 8 )// -7
```

- 无符号溢出取整问题

```rust
let a:usize = 1;
let mod_a = (a.wrapping_sub(2)) % 8; // 7
```
## Game
1. snake(贪吃蛇)
2. maze(迷宫)
3. catch the animal(抓住动物)


