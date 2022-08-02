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

- pkg

```json
//package.json
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
//other use
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