use image::{ self, imageops::* };

fn main() {
    let img = image::open("/home/anthonyb/projects/eas/EAS-Backend-Server/upload/main.jpeg").unwrap().to_rgba8();

    
    for pix in img.pixels() {
        if pix[0] < 255 {
            println!("{}", pix[1]);
        }
    }
}
