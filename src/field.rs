use core::fmt::{Display, Formatter};
use rand::Rng;
use termion::color;

const COLOR_BLACK: color::AnsiValue = color::AnsiValue(0);
const COLOR_RED: color::AnsiValue = color::AnsiValue(1);
const COLOR_GREEN: color::AnsiValue = color::AnsiValue(2);
const COLOR_YELLOW: color::AnsiValue = color::AnsiValue(3);
const COLOR_BLUE: color::AnsiValue = color::AnsiValue(4);
const COLOR_MAGENTA: color::AnsiValue = color::AnsiValue(5);
const COLOR_CYAN: color::AnsiValue = color::AnsiValue(6);
const COLOR_GREY: color::AnsiValue = color::AnsiValue(8);
const COLOR_LIGHT_BLUE: color::AnsiValue = color::AnsiValue(12);
const RESET_COL: color::Reset = color::Reset;

const fn color_lookup(i: u8) -> color::AnsiValue {
    match i {
        1 => COLOR_LIGHT_BLUE,
        2 => COLOR_GREEN,
        3 => COLOR_CYAN,
        4 => COLOR_BLUE,
        5 => COLOR_MAGENTA,
        6 => COLOR_YELLOW,
        7 => COLOR_BLACK,
        8 => COLOR_GREY,
        _ => COLOR_YELLOW,
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
                        "[{col}{num_bombs}{reset}]",
                        col = color::Fg(color_lookup(*num_bombs)),
                        reset = color::Fg(RESET_COL)
                    )
                }
            }
            Cell::Flagged { .. } => write!(
                f,
                "[{red}F{reset}]",
                red = color::Fg(COLOR_RED),
                reset = color::Fg(RESET_COL)
            ),
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

impl Mode {
    pub fn x_size(&self) -> usize {
        match self {
            Mode::EASY => 9,
            Mode::MEDIUM => 16,
            Mode::HARD => 24,
        }
    }

    pub fn y_size(&self) -> usize {
        match self {
            Mode::EASY => 9,
            Mode::MEDIUM => 16,
            Mode::HARD => 24,
        }
    }

    pub fn bombs(&self) -> usize {
        match self {
            Mode::EASY => 10,
            Mode::MEDIUM => 40,
            Mode::HARD => 99,
        }
    }
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
        let space = self.height.checked_ilog10().unwrap_or(0) as usize + 1;
        let mut top = " ".repeat(space + 1);
        let mut nums = " ".repeat(space + 1);
        for x in 0..self.width {
            if x > 9 {
                top.push_str(&format!(" {} ", x / 10));
            } else {
                top.push_str("   ");
            }
            nums.push_str(&format!(" {} ", x % 10));
        }
        if !top.trim().is_empty() {
            write!(f, "{}\n\r", top)?;
        }
        write!(f, "{}\n\r", nums)?;
        let mut cells = 0;

        for y in 0..self.height {
            let chars = y.checked_ilog10().unwrap_or(0) as usize + 1;
            let mut line = format!("{}", y);
            line.push_str(&" ".repeat(space - chars + 1));

            for x in 0..self.width {
                let cell = self.get(x, y).unwrap();
                if let Cell::Flagged { is_bomb: _ } = cell {
                    cells += 1;
                }
                line.push_str(&format!("{}", cell));
            }

            line.push_str(&" ".repeat(space - chars + 1));
            line.push_str(&format!("{}", y));
            write!(f, "{}\n\r", line)?;
        }

        if !top.trim().is_empty() {
            write!(f, "{}\n\r", top)?;
        }
        write!(f, "{}\n\r", nums)?;
        write!(f, "Bombs: {}, Flags: {}\n\r", self.num_mines, cells)?;
        Ok(())
    }
}

/// test if (x, y) is near (a, b)
fn near(x: usize, y: usize, a: usize, b: usize, tol: usize) -> bool {
    x + tol >= a && x <= a + tol && y + tol >= b && y <= b + tol
}

impl Field {
    /// Create a new Field with the preferred Game Mode
    pub fn new(mode: Mode) -> Field {
        let width = mode.x_size();
        let height = mode.y_size();
        let num_mines = mode.bombs();

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
            let x = rand::thread_rng().gen_range(0..self.width);
            let y = rand::thread_rng().gen_range(0..self.width);

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
        Ok(&self.cells[x + y * self.width])
    }

    /// Returns a mutable reference to the cell at position `(x, y)`.
    /// if position is out of bounds, an Error message is returned.
    fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut Cell, String> {
        if x > self.height || y > self.width {
            return Err(String::from("Access out of Bounds"));
        }
        Ok(&mut self.cells[x + y * self.width])
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
        !self.cells.iter().any(|c| matches!(c, Cell::Empty))
    }
}
