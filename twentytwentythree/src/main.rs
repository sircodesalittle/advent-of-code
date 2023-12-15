use std::fs;
mod puzzles;
use crate::puzzles::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => panic!("no args"),
        _ => {
            let day = args[1].as_str();
            let data = fs::read_to_string(format!("data/{}.txt", day)).unwrap();
            match day {
                "day13" => {
                    day13::solve(data);
                }
                _ => panic!("Invalid day argument"),
            }
        }
    }
}
