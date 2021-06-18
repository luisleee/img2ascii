extern crate image;
use image::io::Reader as ImageReader;

fn main() {
    // let mut filename = String::new();
    // std::io::stdin().read_line(&mut filename).unwrap();
    // println!("{}", filename);

    let img = ImageReader::open("mandel.png").unwrap();
    let data = img.decode().unwrap();
    let rgb = data.into_rgb8();
    let raw = rgb.into_raw();
    println!("{:?}", raw);
}
