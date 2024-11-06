#[allow(dead_code)]
struct Structure(i32);
fn main() {
    println!("xxx");
    let _number = 1.0;
    let _width = 5;
    // let number = 1;
    println!("{number:>width$}", width = 10, number = 88);
}
