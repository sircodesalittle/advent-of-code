use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct PossibleGame {
    red: i32,
    blue: i32,
    green: i32,
}

impl PossibleGame {
    fn product(&self) -> i32 {
        return self.red * self.blue * self.green;
    }
}

fn build_possible_game(game_input: &str) -> PossibleGame {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    let drawings = game_input.split("; ");
    for drawing in drawings {
        for drawing_color in drawing.split(", ") {
            let (color_value, color) = drawing_color.split_once(" ").unwrap();
            let temp_value = color_value.parse::<i32>().unwrap();
            if color == "blue" {
                if temp_value > blue {
                    blue = temp_value;
                }
            }
            if color == "red" {
                if temp_value > red {
                    red = temp_value;
                }
            }
            if color == "green" {
                if temp_value > green {
                    green = temp_value;
                }
            }
        }
    }

    PossibleGame {
        red: (red),
        blue: (blue),
        green: (green),
    }
}

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
    println!("Total: {}", id_sum);
}

fn process_line(str: String) -> i32 {
    let (_, game_input) = str.rsplit_once(": ").unwrap();
    return build_possible_game(game_input).product();
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
