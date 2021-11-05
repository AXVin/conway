#![allow(dead_code)]
use rand;
use std::{cmp::PartialEq, convert::TryInto, fmt, thread, time::Duration};

const WIDTH: u32 = 50;
const HEIGHT: u32 = 25;
const DURATION: u64 = 20;
const ALIVE_CHANCE: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Cell::Dead => "░",
            Cell::Alive => "█",
        };
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

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut universe = "".to_string();
        for row in &self.cells {
            for cell in row {
                universe.push_str(&cell.to_string());
            }
            universe.push_str("\n");
        }
        write!(f, "{}", universe)
    }
}

impl Universe {
    fn new(height: u32, width: u32) -> Self {
        let mut cells = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                let rand_number = rand::random::<u32>();
                let cell = if rand_number % ALIVE_CHANCE == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
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

    fn static_live_neighbour_count(&self, cell_row: i32, cell_col: i32) -> u8 {
        let mut count = 0u8;
        for ncell in NEIGHBOURING_CELLS.clone().iter() {
            let row = cell_row + ncell.0;
            let col = cell_col + ncell.1;
            if row < 0 {
                continue;
            } else if row >= self.height.try_into().unwrap() {
                continue;
            }
            let row: usize = row.try_into().unwrap();

            if col < 0 {
                continue;
            } else if col >= self.width.try_into().unwrap() {
                continue;
            }
            let col: usize = col.try_into().unwrap();

            let cell: Cell = self.cells[row][col];
            if cell == Cell::Alive {
                count += 1;
            }
        }
        count
    }

    fn live_neighbour_count(&self, cell_row: i32, cell_col: i32) -> u8 {
        let mut count = 0u8;
        for ncell in NEIGHBOURING_CELLS.clone().iter() {
            let row = cell_row + ncell.0;
            let col = cell_col + ncell.1;
            let row: usize = if row < 0 {
                (self.height - 1) as usize
            } else if row >= self.height.try_into().unwrap() {
                0
            } else {
                row as usize
            };
            let col = if col < 0 {
                (self.width - 1) as usize
            } else if col >= self.width.try_into().unwrap() {
                0
            } else {
                col as usize
            };
            let cell = self.cells[row][col];
            if cell == Cell::Alive {
                count += 1;
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let alive_nearby_cells = self.live_neighbour_count(row as i32, col as i32);
                let cell = cells[row as usize][col as usize];
                let next_cell = match (cell, alive_nearby_cells) {
                    (Cell::Alive, x) if x < 2 || x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (cell, _) => cell.clone(),
                };
                cells[row as usize][col as usize] = next_cell;
            }
        }
        self.cells = cells;
    }

    fn run(&mut self) {
        loop {
            self.tick();
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", self);
            thread::sleep(Duration::from_millis(DURATION));
        }
    }
}

fn main() {
    let mut universe = Universe::new(HEIGHT, WIDTH);
    universe.run();
}
