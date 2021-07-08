extern crate image;
use image::io::Reader as ImageReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut args = std::env::args().skip(1);
    if args.len() < 3 {
        println!("Usage: img2ascii [image] [style] [blocksize]");
        println!("Example: img2ascii image.jpg gr 50");
        std::process::exit(1);
    }

    let img = ImageReader::open(args.next().unwrap()).unwrap();
    let data = img.decode().unwrap();
    let gray = data.into_luma8();

    let mut charlist =
        File::open(String::from("charlist/") + &args.next().unwrap() + &String::from(".txt"))
            .unwrap();

    let size = args.next().unwrap().as_str().parse::<u32>().unwrap();

    let w = gray.width();
    let h = gray.height();
    let mut wb = w / size;
    let mut hb = h / size;

    let mut res = Vec::<u32>::new();

    if w % size != 0 && h % size == 0 {
        wb += 1;
    } else if w % size != 0 && h % size == 0 {
        hb += 1;
    } else if w % size != 0 && h % size != 0 {
        wb += 1;
        hb += 1;
    }
    for _ in 0..wb * hb {
        res.push(0);
    }

    for i in 0..h {
        for j in 0..w {
            let pix = gray.get_pixel(j, i);
            let idx = (i / size) * wb + (j / size);
            res[idx as usize] += pix[0] as u32;
        }
    }

    let mut char_str = String::new();
    charlist.read_to_string(&mut char_str).unwrap();

    for i in 0..res.len() {
        let x = res[i] / (size * size) / 16;
        print!("{}", char_str.chars().nth(x as usize).unwrap());
        if (i + 1) % (wb as usize) == 0 {
            print!("\n");
        }
    }
}
