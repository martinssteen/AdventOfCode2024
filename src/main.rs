#![feature(tuple_trait)]

use chrono;
use chrono::Datelike;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day14;

fn main() {
    let day = get_day();
    println!("Advent of Code 2024 - Day {}", day);
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        14 => day14::day14(),
        _ => println!("This day4 does not exist yet!"),
    }
}

fn get_day() -> u32 {
    let args: Vec<String> = env::args().collect();
    args.last()
        .unwrap()
        .parse::<u32>()
        .unwrap_or(chrono::offset::Local::now().day())
}
