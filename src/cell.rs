pub struct Cell {
    dead: bool,
    pub x: u32,
    pub y: u32,
    pub changed: bool
}

impl Cell {
    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn new(x: u32, y: u32) -> Self {
        Cell {dead: true, x, y, changed: true}
    }

    pub fn kill(&mut self) {
        if !self.dead {
            self.dead = true;
            self.changed = true;
        }
    }

    pub fn revive(&mut self) {
        if self.dead {
            self.dead = false;
            self.changed = true;
        }
    }
}