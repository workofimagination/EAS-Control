use std::{time::Duration, thread};

use crate::stepper::Stepper;
use cursor::Cursor;
use rppal::gpio::{Gpio, OutputPin};

use crate::parser::Coordinate;

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
            micro_delay: Duration::from_micros(25),
        }
    }

    pub fn set_zero(&mut self) { self.cursor.zero(); }

    pub fn go_zero(&mut self) { self.go_position(0.0, 0.0)  }

    pub fn draw_coordinates(&mut self, coordinates: Vec<Coordinate>) {
        for coordinate in coordinates {
            println!("{:?}, {:?}", coordinate.x, coordinate.y);
            self.go_position(coordinate.x, coordinate.y);
        }
    }


    pub fn go_position(&mut self, to_x: f32, to_y: f32) {
        //TODO add horizontal logic.
        self.angled_line(to_x, to_y);
    }

    pub fn rec_line(&mut self, to_x: f32, to_y: f32) {
        let precision: f32 = 0.0625; // 1/16
        let divisions = (1.0/precision) as usize;

        let mut run = to_x - self.cursor.x;
        let mut rise = to_y - self.cursor.y;

        let mut run_dir: bool = true;
        let mut rise_dir: bool = true;

        if run < 0.0 { run_dir = false; run = run * -1.0; }
        if rise < 0.0 { rise_dir = false; rise = rise * -1.0; }
        for step in 1..=(run*divisions as f32) as usize {
            self.x_motor.step(run_dir);
            self.cursor.x += if run_dir { precision } else { -precision };
            self.micro_delay();
            self.reset();
            self.micro_delay();
        }
        for step in 1..=(rise*divisions as f32) as usize {
            self.y_motor.step(rise_dir);
            self.cursor.y += if rise_dir { precision } else { -precision };
            self.micro_delay();
            self.reset();
            self.micro_delay();            
        }
    }

    fn angled_line(&mut self, to_x: f32, to_y: f32) {
        // Position to position v1.1 - Eric
        let precision: f32 = 0.0625; // 1/16
        let divisions = (1.0/precision) as usize;

        let mut run = to_x - self.cursor.x;
        let mut rise = to_y - self.cursor.y;

        let mut run_dir: bool = true;
        let mut rise_dir: bool = true;

        if run < 0.0 { run_dir = false; run = run * -1.0; }
        if rise < 0.0 { rise_dir = false; rise = rise * -1.0; }

        let real_run = run.clone();
        let real_rise = rise.clone();
        let run_larger: bool = if run > rise {
            rise = rise / run;
            run = 1.0;
            true
        } else {
            run = run / rise;
            rise = 1.0;
            false
        };

        let dependent_step_count: usize = {
            let mut count: usize = 0;
            let mut value: f32 = 0.0;
            if (run_larger) {
                while value < rise {
                    count += 1;
                    value += precision;
                }
                count
            } else {
                while value < run {
                    count += 1;
                    value += precision;
                }
                count
            }
        };

        let mut sum_run: f32 = 0.0; 
        let mut sum_rise: f32 = 0.0;
        while if run_larger { sum_run < real_run } else { sum_rise < real_rise } {
            let mut remaining_steps = dependent_step_count;
            if (run_larger) {
                for step in 1..=divisions {
                    self.step_x(run_dir);
                    if remaining_steps > 0 {
                        self.step_y(rise_dir);
                        self.cursor.y += if rise_dir { precision } else { -precision };
                        sum_rise += precision;
                        remaining_steps -= 1;
                    }
                    sum_run += precision;
                    self.cursor.x += if run_dir { precision } else { -precision };
                    self.micro_delay();
                    self.reset();
                    self.micro_delay();                    
                }
            } else {
                for step in 1..=divisions {
                    self.step_y(rise_dir);
                    if remaining_steps > 0 {
                        self.step_x(run_dir);
                        self.cursor.x += if run_dir { precision } else { -precision };
                        sum_run += precision;
                        remaining_steps -= 1;
                    }
                    sum_rise += precision;
                    self.cursor.y += if rise_dir { precision } else { -precision };
                    self.micro_delay();
                    self.reset();
                    self.micro_delay();
                }
            }
        }
        
    }

    fn step_x(&mut self, dir: bool) { self.x_motor.step(dir) }
    fn step_y(&mut self, dir: bool) { self.y_motor.step(!dir) }

    fn step_delay(&self) { thread::sleep(self.step_delay); }
    fn micro_delay(&self) { thread::sleep(self.micro_delay) }

    fn reset(&mut self) {
        self.y_motor.reset();
        self.x_motor.reset();
    }

}





