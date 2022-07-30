fn message_dereference2() {
    let mut message = String::from("Hello");
    let message_2 = &mut message;
    let message_3 = &message_2;
    message_3.push_str(" Jxlust");
}
fn message_dereference() {
    let mut message = String::from("Hello");
    let message_2 = &mut message;
    (*message_2).push_str(" Jxlust!");
    message_2.push_str(" Jxlust!");
    println!("{}", message_2)
}

fn number_reference() {
    // a      <-        b   <-  c
    // 10(address)     ptr      ptr
    let a = 10;
    let b = &a;
    let mut c = &b;
    let d = b;
    // println!("Value of a: {}", &a);
    // println!("Address of a: {:p}", &a); //直接取址
    // println!("Value of b: {:p}", b);
    // println!("Value of c: {:p}", c);
    // println!("Value of d: {:p}", d);
    // 100      ?       e
    // 100(ad) <-  ptr <-  ptr
    let e = &&100; //
    let c = e;
    println!("Value of c: {:p}", e);
    println!("Value of e: {:p}", c);
    println!("Value of *c: {:p}", *c); //指向的也就是100的地址
                                       // 判断一下100的值对不对
    println!("Address of 100: {:p}", &(**e));

    // println!("{}",c)
    // **c = 100;//error
    // println!("{}", a == **c);
    // println!("a address:{:p}",&a);
    // println!("b: {:p}", b);
}
