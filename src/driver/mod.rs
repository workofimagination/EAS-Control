use std::{time::Duration, thread};

use crate::stepper::Stepper;
use cursor::Cursor;

pub mod cursor;
pub struct MotorDriver {
    x_motor: &mut Stepper,
    y_motor: &mut Stepper,
    cursor: &Cursor,
    delay: Duration,
}

impl MotorDriver {
    pub fn new(x_motor: &mut Stepper, y_motor: &mut Stepper, cursor: &Cursor) -> Self{
        Self {x_motor, y_motor, cursor, delay: Duration::from_micros(500)}
    }

    pub fn set_zero(&mut self) {
        self.cursor.zero();
    }

    pub fn go_zero() {
    }

    pub fn go_position(&mut self, to_x: usize, to_y: usize) {
        let rise: isize = to_y - self.cursor.x;
        let run: isize = to_x - self.cursor.y;
    }

    fn move_up(&mut self) {self.y_motor.step(true)}
    fn move_down(&mut self) {self.y_motor.step(false)}
    fn move_left(&mut self) {self.x_motor.step(false)}
    fn move_right(&mut self) {self.x_motor.step(true)}

    fn reset(&mut self) {
        thread::sleep(self.delay); // allow motor to move from move()
        self.y_motor.reset();
        self.x_motor.reset();
        thread::sleep(self.delay); // allow motor to move from motor.reset();
    }


}
