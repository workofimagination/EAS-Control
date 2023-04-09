use image::{ self, imageops::{resize, FilterType::Nearest}, ImageBuffer, Rgba };

fn main() {
    let mut img = image::open("/home/anthonyb/projects/eas/EAS-Backend-Server/upload/main.jpeg").unwrap().to_rgba8();
    resize(&mut img, 100, 100, Nearest);
    img = black_and_white(&img);

    img.save("temp.jpeg").unwrap();
}

fn black_and_white(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut image = image.clone();

    for pix in image.pixels_mut() {
        if pix[0] < 100 {
            for i in 0..3 { 
                pix[i] = 0
            }
        } else {
            for i in 0..3 {
                pix[i] = 255
            }
        }
    }
    
    return image;
}
