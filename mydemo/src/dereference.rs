
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