use image::{ self, imageops::{resize, FilterType::Nearest} };
mod image_process;
use crate::image_process::ImageProcess;

fn main() {
    let img = image::open("/home/anthonyb/projects/eas/EAS-Backend-Server/upload/main.jpeg")
                        .unwrap()
                        .to_rgba8();

    let raw = ImageProcess::black_and_white(&img);

    for p in raw {
        println!("{}", p);
    }
}



