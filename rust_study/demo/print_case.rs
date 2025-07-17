fn print_hello() {
    let last_str = "Hello World";
    let list = ["A", "B", "C", last_str];
    for item in list.iter() {
        println!("{}", &item);
    }

    let (a, mut b) = (true, false);
    b = true;
    assert_eq!(a, b);
}
fn main() {
    print_hello();
}
