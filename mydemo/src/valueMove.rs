fn main(){
    let message = String::from("Hello");//message comming into the scope
    print_message(message);// message is moved into the print_message function
    //message is no longer valid
}
//a is going out of the scope 
//but nothing more will happen because it was moved into print_message

fn print_message(a: String) {
    println!("{}",a);
    let c = a;// c is comming into the scope and a is moved into the c;
    // println!("{}",a); valid
    // a is no longer valid
}
// a is going out of the scope, but nothing more will happen because it was moved

//c is going out of the scope and 'drop'
//is called with clears the memory from the heap


/** move return use start  */
fn main2() {
    // let mut message = String::from("Hello");
    let message = String::from("Hello");
    let message = extend_message(message);

    //基本数据类型是copy
    let age = 30;
    let add_age = extend_age(age);
    println!("{}", age);//30
    println!("{}", add_age);//40
    println!("{}", message);//Hello World!
}

fn extend_message(mut a: String) -> String {
    a.push_str(" World!");
    a
}
fn extend_age(mut a:i32){
    a += 10;
    a
}
/** move return use end  */

