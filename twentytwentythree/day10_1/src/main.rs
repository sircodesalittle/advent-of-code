use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*

.....
.S-7.
.|.|.
.L-J.
.....

.....
.012.
.1.3.
.234.
.....

Pipe
    Connecting Pipe 1
    Connecting Pipe 2
*/

#[derive(Debug, Clone, Copy)]
struct Pipe {
    letter: char,
    distance: u32,
    x: u32,
    y: u32,
}

fn find_element_position_by_x_y(pipes: &Vec<Pipe>, x: u32, y: u32) -> Option<usize> {
    return pipes.iter().position(|p| p.x == x && p.y == y);
}

fn find_element_position_by_letter(pipes: &Vec<Pipe>, letter: char) -> usize {
    return pipes.iter().position(|p| letter == p.letter).unwrap();
}

fn get_up_pipe(pipes: &Vec<Pipe>, current_pipe: &Pipe) -> Option<usize> {
    if current_pipe.y == 0 {
        return None;
    }
    return find_element_position_by_x_y(pipes, current_pipe.x, current_pipe.y - 1);
}

fn get_down_pipe(pipes: &Vec<Pipe>, current_pipe: &Pipe) -> Option<usize> {
    return find_element_position_by_x_y(pipes, current_pipe.x, current_pipe.y + 1);
}

fn get_right_pipe(pipes: &Vec<Pipe>, current_pipe: &Pipe) -> Option<usize> {
    return find_element_position_by_x_y(pipes, current_pipe.x + 1, current_pipe.y);
}

fn get_left_pipe(pipes: &Vec<Pipe>, current_pipe: &Pipe) -> Option<usize> {
    if current_pipe.x == 0 {
        return None;
    }
    return find_element_position_by_x_y(pipes, current_pipe.x - 1, current_pipe.y);
}

fn get_next_pipe_position(
    pipes: &Vec<Pipe>,
    current_pipe: &Pipe,
    enter_direction: String,
) -> (Option<usize>, Option<String>) {
    let value = match current_pipe.letter {
        '|' => match enter_direction.as_str() {
            "bottom" => (
                get_up_pipe(pipes, current_pipe),
                Some(String::from("bottom")),
            ),
            "top" => (
                get_down_pipe(pipes, current_pipe),
                Some(String::from("top")),
            ),
            _ => (None, None),
        },
        '-' => match enter_direction.as_str() {
            "left" => (
                get_right_pipe(pipes, current_pipe),
                Some(String::from("left")),
            ),
            "right" => (
                get_left_pipe(pipes, current_pipe),
                Some(String::from("right")),
            ),
            _ => (None, None),
        },
        'L' => match enter_direction.as_str() {
            "top" => (
                get_right_pipe(pipes, current_pipe),
                Some(String::from("left")),
            ),
            "right" => (
                get_up_pipe(pipes, current_pipe),
                Some(String::from("bottom")),
            ),
            _ => (None, None),
        },
        'J' => match enter_direction.as_str() {
            "top" => (
                get_left_pipe(pipes, current_pipe),
                Some(String::from("right")),
            ),
            "left" => (
                get_up_pipe(pipes, current_pipe),
                Some(String::from("bottom")),
            ),
            _ => (None, None),
        },
        '7' => match enter_direction.as_str() {
            "left" => (
                get_down_pipe(pipes, current_pipe),
                Some(String::from("top")),
            ),
            "bottom" => (
                get_left_pipe(pipes, current_pipe),
                Some(String::from("right")),
            ),
            _ => (None, None),
        },
        'F' => match enter_direction.as_str() {
            "right" => (
                get_down_pipe(pipes, current_pipe),
                Some(String::from("top")),
            ),
            "bottom" => (
                get_right_pipe(pipes, current_pipe),
                Some(String::from("left")),
            ),
            _ => (None, None),
        },
        'S' => {
            panic!("back at S");
        }
        _ => (None, None),
    };
    // dbg!(current_pipe);
    // dbg!(enter_direction);
    // dbg!(&value);
    return value;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut pipes = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for (y, line) in lines.enumerate() {
            if let Ok(ready_line) = line {
                for (x, character) in ready_line.chars().enumerate() {
                    let new_pipe = Pipe {
                        letter: character,
                        distance: 0,
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    pipes.push(new_pipe);
                }
            }
        }
    }

    let start_position = find_element_position_by_letter(&pipes, 'S');
    let start_pipe = pipes.get(start_position).unwrap();
    dbg!(start_pipe);
    let mut back_at_start = false;
    let mut distance_traveled = 0;
    // let pipes_to_use = pipes.clone();

    if let Some(pipe_above_location) = get_up_pipe(&pipes, start_pipe) {
        let pipe_above = pipes.get(pipe_above_location).unwrap();
        let mut current_pipe = pipe_above;
        let mut entry_type = String::from("bottom");
        distance_traveled += 1;
        while !back_at_start {
            if let (Some(next_pipe_position), Some(next_entry_type)) =
                get_next_pipe_position(&pipes, &current_pipe, entry_type.to_string())
            {
                let next_pipe = pipes.get(next_pipe_position).unwrap(); // could be a problem
                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                } else {
                    entry_type = next_entry_type.to_string();
                    current_pipe = next_pipe;
                }
            } else {
                // dead end
                distance_traveled = 0;
                dbg!("dead end");
                back_at_start = true;
            }
        }
    } else {
        // restart
    }

    back_at_start = false;

    if let Some(pipe_below_location) = get_down_pipe(&pipes, &start_pipe) {
        let pipe_below = pipes.get(pipe_below_location).unwrap();
        let mut current_pipe = pipe_below;
        let mut entry_type = String::from("top");
        while !back_at_start {
            if let (Some(next_pipe_position), Some(next_entry_type)) =
                get_next_pipe_position(&pipes, &current_pipe, entry_type.to_string())
            {
                let next_pipe = pipes.get(next_pipe_position).unwrap(); // could be a problem
                                                                        // dbg!(next_pipe);
                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                } else {
                    entry_type = next_entry_type.to_string();
                    current_pipe = next_pipe;
                }
            } else {
                // dead end
                distance_traveled = 0;
                dbg!("dead end");
                back_at_start = true;
            }
        }
    } else {
        // restart
    }

    back_at_start = false;

    if let Some(pipe_right_location) = get_right_pipe(&pipes, &start_pipe) {
        let pipe_right = pipes.get(pipe_right_location).unwrap();
        let mut current_pipe = pipe_right;
        let mut entry_type = String::from("left");
        while !back_at_start {
            if let (Some(next_pipe_position), Some(next_entry_type)) =
                get_next_pipe_position(&pipes, &current_pipe, entry_type.to_string())
            {
                let next_pipe = pipes.get(next_pipe_position).unwrap(); // could be a problem
                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                } else {
                    entry_type = next_entry_type.to_string();
                    current_pipe = next_pipe;
                }
            } else {
                // dead end
                distance_traveled = 0;
                dbg!("dead end");
                back_at_start = true;
            }
        }
    } else {
        // restart
    }

    back_at_start = false;

    if let Some(pipe_left_location) = get_left_pipe(&pipes, &start_pipe) {
        let pipe_left = pipes.get(pipe_left_location).unwrap();
        let mut current_pipe = pipe_left;
        let mut entry_type = String::from("right");
        while !back_at_start {
            if let (Some(next_pipe_position), Some(next_entry_type)) =
                get_next_pipe_position(&pipes, &current_pipe, entry_type.to_string())
            {
                let next_pipe = pipes.get(next_pipe_position).unwrap(); // could be a problem
                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                } else {
                    entry_type = next_entry_type.to_string();
                    current_pipe = next_pipe;
                }
            } else {
                // dead end
                distance_traveled = 0;
                dbg!("dead end");
                back_at_start = true;
            }
        }
    } else {
        // restart
    }

    // list.push_front(start_pipe);

    // check for start pipe neighbors - only two options
    // look at pipe above start
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
