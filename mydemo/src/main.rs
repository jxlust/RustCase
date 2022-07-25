//引用/借用 指针 
fn main() {
    // a      <-        b   <-  c
    // 10(address)     ptr      ptr
    let a = 10;
    let b = &a;
    let c = &b;
    
    // **c = 100;//error
 
    println!("{}", a == **c);

    // println!("a address:{:p}",&a);
    // println!("b: {:p}", b);
}

// fn test() {
//     assert_eq!("1","11");
// }

