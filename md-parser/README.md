## 编写 rust 库

提供给 web 使用，使用 wasm-pack 打包

## 安装 wasm-bindgen-cli

此工具是一个用于促进 Wasm 模块和 JavaScript 之间高级交互的 Rust 库和 CLI 工具。

```shell
cargo install wasm-bindgen-cli
# 安装指定版本
cargo add wasm-bindgen@0.2.91  

```

## 代码里面标记

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("[manulaRustModule] Hello, {}!", name));
}
```

## 安装 wasm-pack

通过 wasm-pack 可以将 Rust 库打包成 wasm 文件，并提供给 web 使用。

```shell
cargo install wasm-pack
# 打包为 web 版本
wasm-pack build --target web

```
