fn main() {
    let message = "hello jxlust 123";
    let message_2  = print_welcome(message);
    println!("{}",message_2);
}
fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    let new_message = "Hi rust";
    // return new_message;
    new_message
}
