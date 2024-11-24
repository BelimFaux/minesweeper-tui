use std::io;

use minesweeper::*;

fn get_int(msg: &str) -> usize {
    println!("{msg}");
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let trimmed = buffer.trim();
        match trimmed.parse::<usize>() {
            Ok(i) => return i,
            Err(..) => {
                println!("Input is not an Integer. Try again.");
                continue;
            }
        }
    }
}

fn get_char(msg: &str) -> char {
    println!("{msg}");
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let trimmed = buffer.trim();
        match trimmed.parse::<char>() {
            Ok(i) => return i,
            Err(..) => {
                println!("Input is not an Integer. Try again.");
                continue;
            }
        }
    }
}

fn main() {
    let mut field = Field::new(Mode::EASY);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clean screen
        println!("{}", field);

        let choose = get_char("Do you want to uncover a cell or place a flag? (u/f)");

        if choose == 'u' {
            let x = get_int("Input X Coordinate: ");
            let y = get_int("Input Y Coordinate: ");

            let bomb = match field.click(x, y) {
                Ok(b) => b,
                Err(s) => {
                    println!("{}", s);
                    continue;
                }
            };

            if !bomb {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clean screen
                println!("Youve hit a bomb.\nGAME OVER.");
                field.uncover();
                println!("{}", field);
                break;
            }
        } else if choose == 'f' {
            let x = get_int("Input X Coordinate: ");
            let y = get_int("Input Y Coordinate: ");

            if let Err(s) = field.toggle_flag(x, y) {
                println!("{}", s);
                continue;
            };
        } else {
            println!("Invalid Input! Try again.");
            continue;
        }

        if field.won() {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clean screen
            println!("You won!\nCongratulations.");
            field.uncover();
            println!("{}", field);
            break;
        }
    }
}
