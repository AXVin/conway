use std::{convert::TryInto, fmt};

#[allow(dead_code)]

struct Cell {
    row: u32,
    col: u32,
    dead: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = if self.dead { "-" } else { "+" };
        write!(f, "{}", text)
    }
}

const NEIGHBOURING_CELLS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

impl Universe {
    fn new(height: u32, width: u32) -> Self {
        let mut cells = Vec::new();
        for row_index in 0..height {
            let mut row = Vec::new();
            for col_index in 0..width {
                let cell = Cell {
                    row: row_index,
                    col: col_index,
                    dead: false,
                };
                row.push(cell);
            }
            cells.push(row)
        }
        Self {
            width,
            height,
            cells,
        }
    }
    fn live_neighbour_count(&self, cell_row: i32, cell_col: i32) -> u8 {
        let mut count = 0u8;
        for ncell in NEIGHBOURING_CELLS.clone().iter() {
            let row = cell_row + ncell.0;
            let col = cell_col + ncell.1;
            let row: usize = if row < 0 {
                self.height.try_into().unwrap()
            } else if row > self.height.try_into().unwrap() {
                0
            } else {
                row.try_into().unwrap()
            };
            let col = if col < 0 {
                self.width.try_into().unwrap()
            } else if col > self.width.try_into().unwrap() {
                0
            } else {
                col.try_into().unwrap()
            };
            let cell = self.cells.get(row).unwrap().get(col).unwrap();
            if cell.dead == false {
                count += 1;
            }
        }
        count
    }

    fn tick(&mut self) -> Result<(), &str> {
        let total_cells = self.height * self.width;
        let mut dead_cells = 0;
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                let alive_nearby_cells = self.live_neighbour_count(
                    cell.row.try_into().unwrap(),
                    cell.col.try_into().unwrap(),
                );
                if cell.dead {
                    dead_cells += 1;
                    if alive_nearby_cells == 2u8 || alive_nearby_cells == 3u8 {
                        cell.dead = false;
                        dead_cells -= 1;
                    }
                } else {
                    if alive_nearby_cells < 2u8 || alive_nearby_cells < 3u8 {
                        cell.dead = true;
                        dead_cells += 1;
                    }
                }
            }
        }
        if dead_cells == total_cells {
            return Err("Game Over!");
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
