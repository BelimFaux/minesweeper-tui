use core::{fmt::{Display, Formatter}, num};
use std::ops::Range;
use rand::random;

#[derive(Debug, Clone)]
enum Cell {
    EMPTY,
    BOMB,
    UNCOVERED(u8),
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::UNCOVERED(u) => {
                if *u == 0 {
                    write!(f, "[ ]")
                } else {
                    write!(f, "[{u}]")
                }
            },
            _ => write!(f, "[#]"),
        }
    }
}

impl Cell {
    fn uncover(&mut self, num_bombs: u8) {
        *self = Cell::UNCOVERED(num_bombs);
    }

    fn place_bomb(&mut self) {
        *self = Cell::BOMB;
    }
}

pub enum Mode {
    EASY,
    MEDIUM,
    HARD,
}

pub struct Field {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    num_mines: usize,
    initialized: bool,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut nums = String::from("  ");
        for x in 0..self.width {
            nums.push_str(&format!(" {} ", x));
        }
        writeln!(f, "{}", nums)?;

        for y in 0..self.height {
            let mut line = format!("{} ", y);

            for x in 0..self.width {
                line.push_str(&format!("{}", self.get(x, y).unwrap()));
            }

            line.push_str(&format!(" {}", y));
            writeln!(f, "{}", line)?;
        }

        writeln!(f, "{}", nums)?;
        Ok(())
    }
}

/// test if (x, y) is near (a, b)
fn near(x: usize, y: usize, a: usize, b: usize, tol: usize) -> bool {
    return x >= a-tol &&
            x <= a+tol &&
            y >= b-tol && 
            y <= b+tol
}

/// helper for generating a valid range around num
fn get_range(num: usize, tol: usize, max: usize) -> Range<usize> {
    let mut low = 0;
    let mut high = num + tol + 1;

    if tol < num {
        low = num - tol;
    }
    if high >= max {
        high = max - 1;
    }

    low..high
}

impl Field {
    pub fn new(mode: Mode) -> Field {
        let width;
        let height;
        let num_mines;
        match mode {
            Mode::EASY => { width = 9; height = 9; num_mines = 10},
            Mode::MEDIUM => { width = 16; height = 16; num_mines = 40},
            Mode::HARD => { width = 24; height = 24; num_mines = 99},
        };

        Field {
            cells: vec![Cell::EMPTY; width * height],
            width: width,
            height: height,
            num_mines: num_mines,
            initialized: false,
        }
    }

    fn initialize(&mut self, initial_x: usize, initial_y: usize) {
        self.initialized = true;
        let mut mines = 0;
        while mines < self.num_mines {
            let x = rand::random::<usize>() % self.width;
            let y = rand::random::<usize>() % self.height;

            if near(x, y, initial_x, initial_y, 1) {
                continue
            }

            let cell = self.get_mut(x, y).unwrap();
            if matches!(cell, Cell::EMPTY) {
                cell.place_bomb();
                mines += 1;
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> Result<&Cell, String> {
        if x > self.height || y > self.width {
            return Err(String::from("Access out of Bounds"))
        }
        Ok(&self.cells[x + y * self.height])
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut Cell, String> {
        if x > self.height || y > self.width {
            return Err(String::from("Access out of Bounds"))
        }
        Ok(&mut self.cells[x + y * self.height])
    }

    fn count_bombs(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for x in get_range(x, 1, self.width) {
            for y in get_range(y, 1, self.height) {
                if x == 0 && y == 0 {
                    continue;
                }
                if matches!(self.get(x, y).unwrap(), Cell::BOMB) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn click(&mut self, x: usize, y: usize) -> Result<bool, String> {
        if !self.initialized {
            self.initialize(x, y);
        }

        let cell = self.get(x, y)?;
        let mut num_bombs = 0;
        match cell {
            Cell::EMPTY => {
                num_bombs = self.count_bombs(x, y);
            },
            Cell::BOMB => return Ok(false),
            Cell::UNCOVERED(_) => return Err(String::from("This Cell is already uncovered.")),
        }

        self.get_mut(x, y).unwrap().uncover(num_bombs);

        // recurse through all neighboring cells
        if num_bombs == 0 {
            for x1 in get_range(x, 1, self.width) {
                for y1 in get_range(y, 1, self.height) {
                    if x1 == x && y1 == y {
                        continue;
                    }
                    let _ = self.click(x1, y1);
                }
            } 
        }

        Ok(true)
    }

    pub fn won(&self) -> bool {
        return self.cells.iter()
                .all(|c| {
                    match c {
                        Cell::BOMB => false,
                        _ => true,
                    }
                })
    }
}