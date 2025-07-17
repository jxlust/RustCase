struct Point {
    x: i32,
}
fn main() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [3, 4, 5, 6];
    Point { x: e } = Point { x: 10 };
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);
    assert_eq!([1, 2, 3, 5, 10], [a, b, c, d, e], "Failed to assign values");

    let mut x = "abc";
    let x = x.len();
}
