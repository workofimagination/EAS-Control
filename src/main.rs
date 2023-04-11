use image::{ self, imageops::{resize, FilterType::Nearest} };
use driver::{MotorDriver, cursor::Cursor};
use stepper::Stepper;
use crate::image_process::ImageProcess;

mod image_process;
mod driver;
mod stepper;
fn main() {
    let mut cursor: Cursor = Cursor::new(); // Cursor - Virtual pointer to the position of the x-y axis motors.
    let mut x_motor: Stepper = Stepper::new(20, 21); // Stepper struct handles low level stepper movement
    let mut y_motor: Stepper = Stepper::new(8, 7);

    let mut driver: MotorDriver = MotorDriver::new(&mut x_motor, &mut y_motor, &cursor); // 

    let mut line = String::new();
    println!("Set zero");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    

    // let img = image::open("/home/anthonyb/projects/eas/EAS-Backend-Server/upload/main.jpeg")
    //                     .unwrap()
    //                     .to_rgba8();

    // let raw = ImageProcess::black_and_white(&img);
    // for p in raw {
    //     println!("{}", p);
    // }
}



