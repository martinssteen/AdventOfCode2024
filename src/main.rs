use chrono;
use chrono::Datelike;
use std::env;

mod day1;
mod day2;

fn main() {
    let day = get_day();
    println!("Advent of Code 2024 - Day {}", day);
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        _ => println!("This day does not exist yet!"),
    }
}

fn get_day() -> u32 {
    let args: Vec<String> = env::args().collect();
    args.last()
        .unwrap()
        .parse::<u32>()
        .unwrap_or(chrono::offset::Local::now().day())
}
