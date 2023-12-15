use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Coordinate {
    x: u32,
    y: u32,
    character: char,
}

struct Number {
    coordinates: Vec<Coordinate>,
    value: u32,
    // Neighboring symbol ranges?
}

impl Number {
    fn highest_x(&self) -> u32 {
        return self.coordinates.last().unwrap().x + 1;
    }

    fn lowest_x(&self) -> u32 {
        let lowest_x = self.coordinates.first().unwrap().x;
        if lowest_x > 0 {
            return lowest_x - 1;
        }
        return lowest_x;
    }

    fn highest_y(&self) -> u32 {
        return self.coordinates.first().unwrap().y + 1;
    }

    fn lowest_y(&self) -> u32 {
        let lowest_y = self.coordinates.first().unwrap().y;
        if lowest_y > 0 {
            return lowest_y - 1;
        }
        return lowest_y;
    }

    fn is_within_symbol_range(&self, coordinate: &Coordinate) -> bool {
        if coordinate.x <= self.highest_x()
            && coordinate.x >= self.lowest_x()
            && coordinate.y >= self.lowest_y()
            && coordinate.y <= self.highest_y()
        {
            return true;
        }
        return false;
    }
}

impl Coordinate {
    fn is_symbol(&self) -> bool {
        return !self.character.is_digit(10) && self.character != '.';
    }
}

// fn process_coordinates(coordinates: Vec<Coordinate>) {
//     for coordinate in coordinates {
//         if coordinate.is_symbol() {
//             if ()
//         }
//         println!(
//             "X: {}, Y: {}, value: {}, is_symbol?: {}",
//             coordinate.x,
//             coordinate.y,
//             coordinate.character,
//             coordinate.is_symbol()
//         )
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut coordinates = Vec::new();
    let mut numbers = Vec::new();
    let mut line_index = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ready_line) = line {
                let mut in_number = false;
                let mut current_string_number = "".to_owned();
                let mut current_string_number_coordinates = Vec::new();
                for (character_index, line_character) in ready_line.chars().enumerate() {
                    let new_coordinate = Coordinate {
                        y: line_index,
                        x: character_index as u32,
                        character: line_character,
                    };
                    if line_character.is_digit(10) {
                        current_string_number.push(line_character);
                        current_string_number_coordinates.push(new_coordinate);
                        if !in_number {
                            in_number = true;
                        }
                        // on the last character
                        if character_index == ready_line.len() - 1 {
                            println!(
                                "On the last character, so add to  numbers: {}",
                                current_string_number
                            );
                            let new_number = Number {
                                value: current_string_number.parse::<u32>().unwrap(),
                                coordinates: current_string_number_coordinates.to_vec(),
                            };
                            numbers.push(new_number);
                        }
                    } else {
                        if in_number {
                            let new_number = Number {
                                value: current_string_number.parse::<u32>().unwrap(),
                                coordinates: current_string_number_coordinates,
                            };
                            numbers.push(new_number);
                            current_string_number = String::from("");
                            current_string_number_coordinates = Vec::new();
                            in_number = false;
                        }
                        coordinates.push(new_coordinate);
                    }
                }
                line_index += 1;
            }
        }
    }
    let mut total = 0;
    for number in numbers {
        for coordinate in &coordinates {
            if coordinate.is_symbol() {
                if number.is_within_symbol_range(&coordinate) {
                    total += number.value
                }
            }
        }
    }
    println!("Total: {}", total);
    // too low: 83744
    // too low: 265315
    // too low: 444549
    // didn't say: 521979
    // 521979
    // 525911 - correct
    // process_coordinates(coordinates);
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
