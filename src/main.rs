use std::env;
mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <day> <part>");
        return;
    }

    let day: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day");
            return;
        }
    };

    let part: u32 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid part");
            return;
        }
    };

    match day {
        1 => days::day01::run(part),
        2 => days::day02::run(part),
        3 => days::day03::run(part),
        _ => println!("Invalid day"),
    }
}
