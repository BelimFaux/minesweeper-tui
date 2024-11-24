use core::fmt::{Display, Formatter};

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
pub(crate) enum Cell {
    Empty,
    Bomb,
    Flagged { is_bomb: bool },
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
            Cell::Flagged { .. } => write!(f, "[{COLOR_RED}F{RESET_COL}]"),
            _ => write!(f, "[*]"),
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

    fn place_flag(&mut self) -> bool {
        let is_bomb = match self {
            Cell::Bomb => true,
            Cell::Empty => false,
            Cell::Uncovered { .. } => return false,
            Cell::Flagged { is_bomb } => {
                *self = if *is_bomb { Cell::Bomb } else { Cell::Empty };
                return true;
            }
        };

        *self = Cell::Flagged { is_bomb };
        true
    }

    fn is_bomb(&self) -> bool {
        match self {
            Cell::Bomb => true,
            Cell::Flagged { is_bomb } => *is_bomb,
            _ => false,
        }
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

    pub fn uncover(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let bombs = self.count_bombs(x, y);
                let cell = self.get_mut(x, y).unwrap();
                match cell {
                    Cell::Empty => cell.uncover(bombs),
                    Cell::Bomb => {
                        cell.place_flag();
                    }
                    Cell::Flagged { is_bomb } => {
                        if !*is_bomb {
                            cell.uncover(bombs);
                        }
                    }
                    Cell::Uncovered { .. } => continue,
                }
            }
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

    /// test if `(x, y)` is a valid position
    fn valid_pos(&self, x: isize, y: isize) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    /// Counts the number of neighboring bombs at position `(x, y)`
    /// will panic if position is invalid. Only intended for internal use.
    fn count_bombs(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        let dx = [-1, -1, -1, 0, 0, 1, 1, 1];
        let dy = [-1, 0, 1, -1, 1, -1, 0, 1];
        for d in 0..8 {
            let new_x = x as isize + dx[d];
            let new_y = y as isize + dy[d];

            if self.valid_pos(new_x, new_y)
                && self.get(new_x as usize, new_y as usize).unwrap().is_bomb()
            {
                count += 1;
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
            Cell::Uncovered { .. } => return Err(String::from("This Cell is already uncovered.")),
            Cell::Flagged { .. } => {
                return Err(String::from("This Cell is flagged, and cannot be clicked."))
            }
        };

        self.get_mut(x, y).unwrap().uncover(num_bombs);

        // recurse through all neighboring cells
        let dx = [-1, -1, -1, 0, 0, 1, 1, 1];
        let dy = [-1, 0, 1, -1, 1, -1, 0, 1];
        if num_bombs == 0 {
            for d in 0..8 {
                let new_x = x as isize + dx[d]; // has to be isize to account for negative values
                let new_y = y as isize + dy[d];

                if self.valid_pos(new_x, new_y) {
                    let _ = self.click(new_x as usize, new_y as usize);
                }
            }
        }

        Ok(true)
    }

    /// Toggle a Flag.
    /// If Cell wasn't uncovered and doesn't contain a Flag yet, a Flag is placed.
    /// If Cell already contains a Flag, the Flag is removed.
    /// If Cell is Uncovered, an Error messages is returned.
    pub fn toggle_flag(&mut self, x: usize, y: usize) -> Result<(), String> {
        let cell = self.get_mut(x, y)?;

        if !cell.place_flag() {
            Err(String::from("This Field is already uncovered."))
        } else {
            Ok(())
        }
    }

    /// Check if the game is won, i.e. all cells not containing bombs have been uncovered.
    pub fn won(&self) -> bool {
        return !self.cells.iter().any(|c| matches!(c, Cell::Empty));
    }
}
