use std::fmt::Display;

use clap::{Parser, ValueEnum};

/// Type representing Game Mode
/// Values correspond to different field sizes and bomb counts
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Mode {
    Easy,
    Medium,
    Hard,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Easy => f.write_str("easy"),
            Mode::Medium => f.write_str("medium"),
            Mode::Hard => f.write_str("hard"),
        }
    }
}

impl Mode {
    #[must_use]
    pub fn x_size(&self) -> usize {
        match self {
            Mode::Easy => 9,
            Mode::Medium => 16,
            Mode::Hard => 24,
        }
    }

    #[must_use]
    pub fn y_size(&self) -> usize {
        match self {
            Mode::Easy => 9,
            Mode::Medium => 16,
            Mode::Hard => 24,
        }
    }

    #[must_use]
    pub fn bombs(&self) -> usize {
        match self {
            Mode::Easy => 10,
            Mode::Medium => 40,
            Mode::Hard => 99,
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(short, long, default_value_t = Mode::Medium)]
    /// Controls the size of the field and the number of bombs
    pub difficulty: Mode,
}
