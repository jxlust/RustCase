//reference 对象的引用
fn main() {
    let mut message = String::from("Hello");

    //不可变引用
    let message_3 = &message;
    println!("{}", message_3);
    //可变引用
    let message_2 = &mut message;

    unpredictable_mutate(message_2);
    println!("{}", message_2);
    println!("{}", message);
}
fn unpredictable_mutate(val: &mut String) {
    val.push_str("_unpredictable");
}

fn mutable_borrow() {
    let mut message = String::from("Hello");
    let message_2 = &mut message;
    //message_2 指向的是message 的ptr 指针
    message_2.push_str(" Jxlust");

    //&String
    //message_2 is not owner of data
    //message_2 is "borrowing" a reference to message
    println!("{}", message_2);
    println!("{}", message);
}

//等价于
// let ref x = 1;
// let x = &1; //引用

// 等价于
//let &y = x;
// let y = *x;//取值dereference