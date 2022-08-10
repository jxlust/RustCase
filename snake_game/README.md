## wasm-bindgen

```
[dependencies]
wasm-bindgen = "0.2.82"
```

## wasm-pack

```shell
# install wasm-pack
cargo install wasm-pack

# use wasm-pack build into platform
wasm-pack build --target web
```

> question:
> wasm-pack build Installing wasm-bindgen...
> look: https://github.com/rustwasm/wasm-pack-template/issues/44

```shell
cargo install wasm-bindgen-cli --version 0.2.82
```

- cargo install cargo-generate

- pkg, package.json

```json
{
  "name": "snake_game",
  "version": "0.1.0",
  "files": ["snake_game_bg.wasm", "snake_game.js", "snake_game.d.ts"],
  "module": "snake_game.js",
  "types": "snake_game.d.ts",
  "sideEffects": false
}
```

```json
"dependencies": {
    "snake_game": "file:../pkg"
}
```

- package.json miss?
  cargo.toml add this:

```
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

## rust & javascript

1. #[wasm_bindgen]
2. use "extern"

## wee_alloc use

```rust
//wasm-pack build --target web
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

//export fun into wasm
#[wasm_bindgen]
pub fn greet(name: &str) {
    // println!("Hello {}", name);
    alert(name)
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(name: &str);
}
//wasm-pack build --target web

```

## Diretion

```rust
  //取下一列取模
if self.snake.direction == Direction::Right {
    let row_start = row * self.width;
    let next_col = (snake_index + 1) % self.width;
    self.snake.body[0].0 = row_start + next_col;
}

if self.snake.direction == Direction::Left {
    let row_start = row * self.width;
    let next_col = (snake_index - 1) % self.width;
    self.snake.body[0].0 = row_start + next_col;
}
//取下一行取模
if self.snake.direction == Direction::Up {
    let next_row = (row - 1) % self.width;
    self.snake.body[0].0 = next_row * self.width + col;
}
if self.snake.direction == Direction::Down {
    let next_row = (row + 1) % self.width;
    self.snake.body[0].0 = next_row * self.width + col;
}
```

refactor 重构：

```rust
pub fn update(&mut self) {
    let snake_index = self.snake_header();
    let (row, col) = self.index_to_cell(snake_index);
    //左右方向：取下一列(+1/-1)取模
    //上下方向：取下一行(+1/-1)取模
    let (row, col) = match self.snake.direction {
        Direction::Right => (row, (snake_index + 1) % self.width),
        Direction::Left => (row, (snake_index - 1) % self.width),
        Direction::Up => ((row - 1) % self.width, col),
        Direction::Down => ((row + 1) % self.width, col),
    };
    let idx = self.cell_to_index(row, col);
    self.set_snake_header(idx);
}

```

## ptr updated attention

```rust
pub fn change_ptr(&mut self) {
  self.snake.body = vec![SnakeCell(2048)];
}
```

## use case

[https://github.com/jxlust/RustCase/blob/main/snake_game/www/index.ts](https://github.com/jxlust/RustCase/blob/main/snake_game/www/index.ts)
