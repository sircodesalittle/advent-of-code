use regex::Regex;
use std::collections::HashMap;
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut id_sum = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ready_line) = line {
                id_sum += process_line(ready_line);
            }
        }
    }
    // 234 is too low
    println!("Total: {}", id_sum);
}

fn process_drawing(drawing: &str) -> bool {
    let mut current_counts: HashMap<String, i32> = HashMap::new();
    let mut is_valid_game = true;
    for drawing_color in drawing.split(", ") {
        let (color_value, color) = drawing_color.split_once(" ").unwrap();
        current_counts.entry(color.to_owned()).or_insert(0);
        let color_count = current_counts.entry(color.to_owned()).or_insert(0);
        let color_value_int = color_value.parse::<i32>().unwrap();
        *color_count += color_value_int;
    }
    match &current_counts.get("red") {
        Some(count) => {
            if count > &&12 {
                is_valid_game = false;
                println!("\t\tInvalid game - more than 12 red: {}", count);
            }
        }
        None => (),
    }
    match &current_counts.get("green") {
        Some(count) => {
            if count > &&13 {
                is_valid_game = false;
                println!("\t\tInvalid game - more than 13 green: {}", count);
            }
        }
        None => (),
    }
    match &current_counts.get("blue") {
        Some(count) => {
            if count > &&14 {
                is_valid_game = false;
                println!("\t\tInvalid game - more than 14 blue: {}", count);
            }
        }
        None => (),
    }

    return is_valid_game;
}

fn process_game(game_input: &str) -> bool {
    println!("\t{}", game_input);
    let drawings = game_input.split("; ");
    for drawing in drawings {
        let is_valid_game = process_drawing(drawing);
        if !is_valid_game {
            return false;
        }
    }
    return true;
}

fn process_line(str: String) -> i32 {
    println!("{}", str);
    let re = Regex::new(r"Game (?s)(.*):").unwrap();
    let Some(caps) = re.captures(&str) else {
        return 0;
    };
    let game_id = &caps[1];
    let (_, game_input) = str.rsplit_once(": ").unwrap();
    if process_game(game_input) {
        println!("Game {} is a valid game", game_id);
        return game_id.parse::<i32>().unwrap();
    }
    return 0;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
