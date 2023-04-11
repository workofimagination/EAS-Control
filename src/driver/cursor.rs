use core::fmt;

pub struct Cursor {
    pub x: usize,
    pub y: usize,

}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0}
    }

    pub fn zero(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

impl fmt::Debug for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cursor").field("x", &self.x).field("y", &self.y).finish()
    }
}
