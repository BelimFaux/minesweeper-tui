use core::fmt::{Display, Formatter};
use std::ops::Range;

const COLOR_BLACK: &str = "\x1b[30m";
const COLOR_GREY: &str = "\x1b[90m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_LIGHT_BLUE: &str = "\x1b[94m";
const COLOR_MAGENTA: &str = "\x1b[35m";
const COLOR_CYAN: &str = "\x1b[36m";
const RESET_COL: &str = "\x1b[0m";

const fn color_lookup(i: u8) -> &'static str {
    if i == 1 {
        COLOR_LIGHT_BLUE
    } else if i == 2 {
        COLOR_GREEN
    } else if i == 3 {
        COLOR_RED
    } else if i == 4 {
        COLOR_BLUE
    } else if i == 5 {
        COLOR_MAGENTA
    } else if i == 6 {
        COLOR_CYAN
    } else if i == 7 {
        COLOR_BLACK
    } else if i == 8 {
        COLOR_GREY
    } else {
        RESET_COL
    }
}

/// type representing a Cell in the Field.
/// Can be Empty, contain a Bomb or be Uncovered with an amount of surrounding bombs.
#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Bomb,
    Uncovered { num_bombs: u8 },
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Uncovered { num_bombs } => {
                if *num_bombs == 0 {
                    write!(f, "[ ]")
                } else {
                    write!(
                        f,
                        "[{col}{num_bombs}{RESET_COL}]",
                        col = color_lookup(*num_bombs)
                    )
                }
            }
            _ => write!(f, "[#]"),
        }
    }
}

impl Cell {
    /// Change cell to Uncovered with number of bombs.
    fn uncover(&mut self, num_bombs: u8) {
        *self = Cell::Uncovered { num_bombs };
    }

    /// place a bomb in Cell
    fn place_bomb(&mut self) {
        *self = Cell::Bomb;
    }
}

/// Type representing Game Mode
pub enum Mode {
    EASY,
    MEDIUM,
    HARD,
}

/// struct representing Playing Field.
/// contains a vector of cells with size width * height, and a certain number of bombs..
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
    x >= a - tol && x <= a + tol && y >= b - tol && y <= b + tol
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
    /// Create a new Field with the preferred Game Mode
    pub fn new(mode: Mode) -> Field {
        let (width, height, num_mines) = match mode {
            Mode::EASY => (9, 9, 10),
            Mode::MEDIUM => (16, 16, 40),
            Mode::HARD => (24, 24, 99),
        };

        Field {
            cells: vec![Cell::Empty; width * height],
            width,
            height,
            num_mines,
            initialized: false,
        }
    }

    // initialize Field from initial position, by placing `num_bombs` bombs on random positions.
    // The initial position and all neighboring cells are guaranteed to not contain num_bombs
    fn initialize(&mut self, initial_x: usize, initial_y: usize) {
        self.initialized = true;
        let mut mines = 0;
        while mines < self.num_mines {
            let x = rand::random::<usize>() % self.width;
            let y = rand::random::<usize>() % self.height;

            if near(x, y, initial_x, initial_y, 1) {
                continue;
            }

            let cell = self.get_mut(x, y).unwrap();
            if matches!(cell, Cell::Empty) {
                cell.place_bomb();
                mines += 1;
            }
        }
    }

    /// Returns an immutable reference to the cell at position `(x, y)`.
    /// if position is out of bounds, an Error message is returned.
    fn get(&self, x: usize, y: usize) -> Result<&Cell, String> {
        if x > self.height || y > self.width {
            return Err(String::from("Access out of Bounds"));
        }
        Ok(&self.cells[x + y * self.height])
    }

    /// Returns a mutable reference to the cell at position `(x, y)`.
    /// if position is out of bounds, an Error message is returned.
    fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut Cell, String> {
        if x > self.height || y > self.width {
            return Err(String::from("Access out of Bounds"));
        }
        Ok(&mut self.cells[x + y * self.height])
    }

    /// Counts the number of neighboring bombs at position `(x, y)`
    /// will panic if position is invalid. Only intended for internal use.
    fn count_bombs(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for x in get_range(x, 1, self.width) {
            for y in get_range(y, 1, self.height) {
                if x == 0 && y == 0 {
                    continue;
                }
                if matches!(self.get(x, y).unwrap(), Cell::Bomb) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Click cell at position `(x, y)`.
    /// Will return true, and uncover the cell, if the cell was empty, and false if cell contained a bomb.
    /// If cell was empty, and had 0 surrounding bombs, click will recurse and click all
    /// surrounding cells.
    /// If position is invalid or was already clicked, an error message is returned.
    pub fn click(&mut self, x: usize, y: usize) -> Result<bool, String> {
        if !self.initialized {
            self.initialize(x, y);
        }

        let cell = self.get(x, y)?;
        let num_bombs = match cell {
            Cell::Empty => self.count_bombs(x, y),
            Cell::Bomb => return Ok(false),
            Cell::Uncovered { num_bombs: _ } => {
                return Err(String::from("This Cell is already uncovered."))
            }
        };

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

    /// Check if the game is won, i.e. all cells not containing bombs have been uncovered.
    pub fn won(&self) -> bool {
        return !self.cells.iter().any(|c| matches!(c, Cell::Empty));
    }
}
