## 包管理 cargo

```
cargo init
cargo run
```

## 文件目录

```shell
     cargo new hello_rust
```

例子中必须有一个 src/main.rs 文件，因为我们创建的是 binary application。 如果我们创建的是 library（在 cargo new 时添加参数--lib ），那么 cargo 会为我们创建 src/lib.rs。

## 模块化 module

1. super：上层模块 self：当前模块。super 类似：../, self 类似：./
2. 同一层级文件，利用 mod
3. 同一层级文件和文件夹下，必须要有入口文件 mod.rs;
4. path:

- 绝对路径从 crate 根开始，以 crate 名或者字面值 crate 开头
- 相对路径从当前模块开始，以 self、supper 或者当前模块的标识符开头。

5. 与 main.rs 同一层级文件：use mydemo::{Log,Person}; mydemo 是 package name in Cargo.toml;

## 模块
1.  crate 是所有 Rust 代码的根。
2.  抽离下面代码中的模块到 src/my_struct.rs 文件中。文件必须被添加到 src/ 文件夹下，便编译器才能找到。 文件的命名规范使用 snake_case。
3.  use 语句中的 crate:: 部分是因为所有 Rust 项目都是 crate。 Rust 项目可以由多个文件（modules）组成，文件也可以嵌套在文件夹中（也是 modules）。 使用 crate::前缀表示来访问 module 树的根目录。
    3.1 必须使用 mod 引入 module（文件或文件夹）。
    3.2 use 关键字可以方便地将完整限定的类型名称映射到类型名。
4.  Rust 中的可见性与 Java 等语言略有不同，需要记住几条规则：
    - 模块内部的所有内容（即/src 文件夹中的文件或子文件夹）可以访问该模块内的任何其他内容。
    - 模块外部的所有内容只能访问该模块的公共成员。
5.  Modules 文件夹
    5.1 当引用文件夹 module 时，我们使用文件夹名称（就如同对文件 module 一样），但 Rust 需要文件夹中存在一个名为 mod.rs 的文件。/src/foo/mod.rs

```rust
struct MyStruct {}

fn main() {
    let _ms = MyStruct {};
}


```

> > 注意 _ 前缀变量名：如果定义了不使用的变量，Rust 会有警告。所以我们在变量前使用 _ 前缀，它可以通知编译器这是我们故意的，阻止编译器发出警告。 但在日常开发中我们不建议这么使用。

## 机器码文件

```shell
#打印编译main.rs文件:
xxd -g1 main
```

## rust 系统三大核心

1. 借用
2. 所有权
3. 生命周期

## 学习目录
