fn main() {
    let is_it_fun = false;
    //i32 -> signed integer of 32bits
    // 4 * 8bits max: 2^32 - 1
    let num = -10;

    //u8 -> unsigned integer of 8bits
    //2^8 - 1 = 255
    let small_num: u8 = 255;

    //i8 -> signed interger of 8bits
    // -2^7  => 2^7 - 1
    // -128 - 127
    let small_num_2: i8 = 127;

    //operating system related type
    //32 => u32  64 => u64
    // 4bytes  8bytes
    let sys_num: isize = -10;
    let sys_num_2: usize = 1000;

    print_number();
}

fn print_number() {
    //连接符无意义
    let custom_num = 99_000;
    //hex 16进制
    let hex_num = 0xfa;
    //binary
    let bin_num = 0b0010_1011;
    //UTF-8(hex):41 ASCII 16进制的hex, A = 0x41
    let byte_num = b'A';

    println!("{}", custom_num); //99000
    println!("{}", hex_num); //250
    println!("{}", bin_num); //43
    println!("{}", byte_num); //65
}

fn print_float() {
    let float_num: f32 = 3.14;
    let float_num_2 = 3.2233242342343242;

    // let tup = (20, 3.14, 1); 元组
    let tup: (i32, &str, u8) = (20, "Hello", 1);

    println!("{}", tup.1);
    //类似解构
    let (a, b, other) = tup;
    println!("{}", other);
    
    //数组
    let x = [1, 5, 6, 7];
    println!("{}", x[1]);

    let y = [2; 5]; //fill 2 length=5  => [2,2,2,2,2]
    println!("{}", y[4]);
}
