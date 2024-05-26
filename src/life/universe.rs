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
        self.draw();
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

    pub fn draw(self: &Self) {
        for row in 0..self.height {
            self.render_row(row);
        }
    }
    // TODO: add in proper sleep
    // for now, we will render at 1 fps
    pub fn tick(self: &mut Self) {
        let duration = Duration::new(1, 0);
        for _ in 0..self.framerate {
            self.draw();
            self.update();
            print!("\x1B[2J\x1B[1;1H");
            sleep(duration);
        }
    }

    pub fn render_row(&self, row: u32) {
        for cur_col in 0..self.width {
            let idx = row * self.width + cur_col;
            let cur_cell = self.cells.get(idx as usize);
            if let Some(c) = cur_cell {
                c.render();
            }
        }
        println!("");
    }

    fn update(&mut self) {
        todo!()
    }
}

impl Cell {
    fn render(&self) {
        match self {
            Self::Alive => {
                print!("{}", LIVE_CELL);
            }
            Self::Dead => {
                print!("{}", DEAD_CELL);
            }
        }
    }
}