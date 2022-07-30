fn main() {
    //基本数据类型 stack
    let a = 10;
    let b = a;
    let c = 15;
    let d = add(a, b);
    println!("{}", d);

    //指针指向 堆heap
    //指针 长度是3 ptr capacity length
    let message = String::from("Hello");
    // move .... 
    //原stack 位置会清空
    let message_2 = message;
    //canot use meesage because it was moved to message_2
    println!("{}", message);//error
}
//stack
//打印编译main.rs文件: xxd -g1 main
fn add(x: u32, y: u32) -> u32 {
    // let sum = x + y;
    //return sum;
    x + y
}
