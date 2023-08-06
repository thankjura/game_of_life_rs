use std::cell::RefCell;
use crate::cell::Cell;

pub struct GameContext {
    pub cells: Vec<Vec<RefCell<Cell>>>,
    pub size_x: u32,
    pub size_y: u32,
}

impl GameContext {
    pub fn new(size_x: u32, size_y: u32) -> Self {
        let mut out = vec![];

        for x in 0..size_x {
            let mut cells = vec![];
            for y in 0..size_y {
                cells.push(RefCell::new(Cell::new(x, y)));
            }
            out.push(cells);
        }

        Self {
            cells: out,
            size_x,
            size_y
        }
    }

    fn count_living_cells(&self, x: i32, y: i32) -> u8 {
        let mut out = 0;

        let checked_coords = vec![
            (x-1, y-1),
            (x+0, y-1),
            (x+1, y-1),
            (x-1, y+0),
            (x+1, y+0),
            (x-1, y+1),
            (x+0, y+1),
            (x+1, y+1),
        ];

        for (x_coord, y_coord) in checked_coords {
            if x_coord < 0 || x_coord >= self.size_x as i32 {
                continue;
            }
            if y_coord < 0 || y_coord >= self.size_y as i32 {
                continue;
            }

            let cell = self.cells[x_coord as usize][y_coord as usize].borrow();
            if !cell.is_dead() {
                out += 1;
            }
        }

        out
    }

    pub fn set_living(&self, x: u32, y: u32, living: bool) {
        if x >= self.size_x || y >= self.size_y {
            return;
        }
        let mut cell = self.cells[x as usize][y as usize].borrow_mut();
        if living {
            cell.revive();
        } else {
            cell.kill();
        }
    }

    pub fn tick(&self) {
        for line in &self.cells {
            for cell in line {
                let mut cell = cell.borrow_mut();
                let living_cells = self.count_living_cells(cell.x as i32, cell.y as i32);
                match living_cells {
                    3 => {
                        cell.revive();
                    }
                    2 => {

                    }
                    _ => {
                        cell.kill();
                    }
                }
            }
        }
    }
}