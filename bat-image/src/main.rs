// use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::path::Path;
fn main() {
    let path = Path::new("test.jpg");
    // 打开图片
    let img = image::open(path).unwrap();

    println!("dimensions {:?}", img.dimensions());
    img.as_bytes()
}
