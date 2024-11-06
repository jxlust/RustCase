use image::GenericImageView;
use std::path::Path;
fn main() {
    let path = Path::new("test.jpg");
    // 打开图片
    let img = image::open(path).unwrap();

    println!("dimensions {:?}", img.dimensions());

    // 调整图片大小
    let resized_img = img.resize(1280, 120, image::imageops::FilterType::Gaussian);
    resized_img.save("jxl.png").unwrap();
}
