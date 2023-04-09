use image::{ImageBuffer, Rgba};

pub struct ImageProcess {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>
}

impl ImageProcess {
    pub fn black_and_white(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();

        for pix in image.pixels() {
            if pix[0] < 100 {
                vector.push(1);
            } else {
                vector.push(0);
            }
        }
        
        return vector;
    }
}


