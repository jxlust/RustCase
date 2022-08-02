use wasm_bindgen::prelude::*;

//export fun into wasm
#[wasm_bindgen]
pub fn greet(name: &str) {
    println!("Hello {}", name);
}
//wasm-pack build --target web
