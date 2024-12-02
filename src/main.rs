use std::env;
use chrono;
use chrono::Datelike;

mod day1;
mod day2;

fn main() {
    
    day2::day2()
}

// fn get_day() -> u32 {
//     let args: Vec<String> = env::args().collect();
//     return args.first().unwrap().parse::<u32>().unwrap_or(chrono::offset::Local::now().day())
// }
