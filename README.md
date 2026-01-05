# minesweeper-tui

A simple tui minesweeper game.

Features:

- mouse AND keyboard controls
- vim-like controls
- different difficulties
- colored output
- written in Rust (blazingly fast, duh)

## Running

The game can be compiled and installed via [cargo](https://github.com/rust-lang/cargo):

```sh
cargo install --path .
```

Or if you just want to play without installing:

```sh
cargo build --release
./target/release/minesweeper-tui
```

You can control the difficulty by passing it as the `-d/--difficulty` flag when starting the program:

```sh
minesweeper-tui -d easy
```

## Controls

| key                | description                |
| ------------------ | -------------------------- |
| Left click         | Reveal clicked cell        |
| `u` OR Enter       | Reveal cell under cursor   |
| Right click        | Place flag on clicked cell |
| `f` OR Backspace   | Place flag under cursor    |
| `q`                | Quit                       |
| `j` OR Arrow Down  | Move cursor down one cell  |
| `k` OR Arrow Up    | Move cursor up one cell    |
| `h` OR Arrow Left  | Move cursor left one cell  |
| `l` OR Arrow Right | Move cursor right one cell |
