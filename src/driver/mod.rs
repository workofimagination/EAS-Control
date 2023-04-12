use std::{time::Duration, thread};

use crate::stepper::Stepper;
use cursor::Cursor;
use rppal::gpio::{Gpio, OutputPin};

pub mod cursor;
pub struct MotorDriver {
    x_motor: Stepper,
    y_motor: Stepper,
    micro_pin: OutputPin,
    cursor: Cursor,
    step_delay: Duration,
    micro_delay: Duration,
}

impl MotorDriver {
    pub fn new(x_motor: Stepper, y_motor: Stepper, cursor: Cursor, micro_pin: u8) -> Self{
        Self {
            x_motor, 
            y_motor, 
            micro_pin: Gpio::new().unwrap().get(micro_pin).unwrap().into_output(), 
            cursor, 
            step_delay: Duration::from_millis(100) ,
            micro_delay: Duration::from_micros(250),
        }
    }

    pub fn set_zero(&mut self) { self.cursor.zero(); }

    pub fn go_zero(&mut self) { self.go_position(0.0, 0.0) }

    pub fn go_position(&mut self, to_x: f32, to_y: f32) {
        // poisition to position v1 - Eric

        //TODO add horizontal logic.
        self.angled_line(to_x, to_y) 
    }

    pub fn angled_line(&mut self, to_x: f32, to_y: f32) {
        let mut run = to_x - self.cursor.x;
        let mut rise = to_y - self.cursor.y;

        let mut run_dir: bool = true;
        let mut rise_dir: bool = true;

        if run < 0.0 { run_dir = false; run = run * -1.0; }
        if rise < 0.0 { rise_dir = false; rise = rise * -1.0; }

        let real_run = run;
        let real_rise = rise;
        let run_larger: bool = if run > rise {
            rise = rise / run;
            run = 1.0;
            true
        } else {
            rise = 1.0;
            run = run / rise;
            false
        };

        let mut sum_run: f32 = 0.0; 
        let mut sum_rise: f32 = 0.0;
        self.micro_pin.set_high();
        while if run_larger { sum_run < real_run } else { sum_rise < real_rise } {
            if (run_larger) {
                self.step_x(run_dir);
                self.cursor.x += if run_dir { 0.0625 } else { -0.0625 };
                sum_run += 0.0625;
                sum_rise += rise*0.0625;
                if (sum_rise > 0.055) {
                    self.step_y(rise_dir);
                    sum_rise = 0.0;
                    self.cursor.y += if rise_dir { 0.0625 } else { -0.0625 };
                }
            } else {
                self.step_y(rise_dir);
                self.cursor.y += if rise_dir { 0.0625 } else { -0.0625 };
                sum_rise += 0.0625;
                sum_run += run*0.0625;
                if (sum_run > 0.055) {
                    self.step_x(run_dir);
                    sum_run = 0.0;
                    self.cursor.x += if run_dir { 0.0625 } else { -0.0625 };
                }
            }
            self.micro_delay();
            self.reset();
            self.micro_delay();
        }
        self.micro_pin.set_low();
    }

    fn step_x(&mut self, dir: bool) { self.x_motor.step(dir) }
    fn step_y(&mut self, dir: bool) { self.y_motor.step(dir) }

    fn step_delay(&self) { thread::sleep(self.step_delay); }
    fn micro_delay(&self) { thread::sleep(self.micro_delay) }

    fn reset(&mut self) {
        self.y_motor.reset();
        self.x_motor.reset();
    }

}





