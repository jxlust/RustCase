//引用/借用 指针
fn main() {
    let message = "Hello";
    //message is &str, message is "Hello" memory reference. readonly
    //    message.push_str("x"); error
    // message = "1";

    let mut message_2 = String::from("Hello");
    message_2.push_str(" jxlust");
    let message_3 = &message_2;

    let message_4 = String::from("Jxlust");
    let slice = &message_4[2..4];
    let slice_1 = &message_4[2..=4];
    let slice_2 = &message_4[..];

    // move_me(message_4); //error cant move
    // message_4.clear();// error

    println!("{},{},{}", slice, slice_1, slice_2)
}

fn move_me(val: String) {}

// fn test() {
//     assert_eq!("1","11");
// }
fn clone_message() {
    //clone
    let mut message = String::from("Jxlust");
    let message_2 = message.clone();
    message.clear();
    println!("{},{}", message, message_2);
}
