use core::fmt;
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Alive = 1,
    Dead = 0,
}

const DEAD_CELL: &str = "◻";
const LIVE_CELL: &str = "◼";

pub struct Universe {
    framerate: i64,
    width: u32,
    height: u32,
    paused: bool,
    cells: Vec<Cell>,
}

impl fmt::Display for Universe {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        print!("{}", self.draw());
        Ok(())
    }
}

impl Universe {
    pub fn new(framerate: i64, width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            framerate,
            width,
            height,
            cells,
            paused: false,
        }
    }

    pub fn draw(self: &Self) -> String {
        let mut output: Vec<&str> = vec![];
        for row in 0..self.height {
            let mut row = self.render_row(row);
            output.append(&mut row)
        }
        output.join("")
    }

    // TODO: add in proper sleep
    // for now, we will render at 1 fps
    pub fn tick(self: &mut Self) {
        let nanos = (1 / self.framerate) as u32 * (10_u32.pow(9));
        let duration = Duration::new(0, nanos);
        for _ in 0..self.framerate {
            print!("{}", self.draw());
            self.update();
            print!("\x1B[2J\x1B[1;1H");
            sleep(duration);
        }
    }

    pub fn render_row(&self, row: u32) -> Vec<&str> {
        let mut output = vec![];
        for cur_col in 0..self.width {
            let idx = row * self.width + cur_col;
            let cur_cell = self.cells.get(idx as usize);
            if let Some(c) = cur_cell {
                output.push(c.render());
            }
        }
        output.push("\n");
        output
    }
    // NOTE: Updates cells in place
    fn update(&mut self) {
        let mut next_state = vec![];

        for (pos, c) in self.cells.iter().enumerate() {
            let h = pos.div_euclid(self.height as usize) as u32;
            let w = pos.rem_euclid(self.height as usize) as u32;
            let live_count = self.live_neighbor_count(h, w);

            match c {
                Cell::Dead => {
                    if live_count == 3 {
                        next_state.push(Cell::Alive)
                    } else {
                        next_state.push(Cell::Dead)
                    }
                }
                Cell::Alive => {
                    if live_count < 2 {
                        next_state.push(Cell::Dead);
                    }
                    if live_count <= 3 && live_count >= 2 {
                        next_state.push(Cell::Alive);
                    }
                    if live_count >= 4 {
                        next_state.push(Cell::Dead);
                    }
                }
            }
        }

        self.cells = next_state;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl Cell {
    fn render(&self) -> &str {
        match self {
            Self::Alive => LIVE_CELL,
            Self::Dead => DEAD_CELL,
        }
    }
}
