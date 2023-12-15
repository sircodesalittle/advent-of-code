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

    let mut pipes_in_loop = Vec::new();
    let mut done = false;

    let mut pipes = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for (y, line) in lines.enumerate() {
            if let Ok(ready_line) = line {
                for (x, character) in ready_line.chars().enumerate() {
                    let new_pipe = Pipe {
                        letter: character,
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
                pipes_in_loop.push(next_pipe.clone());
                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                    done = true;
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

    if !done {
        back_at_start = false;
    }

    if let Some(pipe_below_location) = get_down_pipe(&pipes, &start_pipe) {
        let pipe_below = pipes.get(pipe_below_location).unwrap();
        let mut current_pipe = pipe_below;
        let mut entry_type = String::from("top");
        while !back_at_start {
            if let (Some(next_pipe_position), Some(next_entry_type)) =
                get_next_pipe_position(&pipes, &current_pipe, entry_type.to_string())
            {
                let next_pipe = pipes.get(next_pipe_position).unwrap(); // could be a problem
                pipes_in_loop.push(next_pipe.clone());

                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                    done = true;
                } else {
                    entry_type = next_entry_type.to_string();
                    current_pipe = next_pipe;
                }
            } else {
                // dead end
                distance_traveled = 0;
                pipes_in_loop = Vec::new();
                dbg!("dead end");
                back_at_start = true;
            }
        }
    } else {
        // restart
    }

    if !done {
        back_at_start = false;
    }

    if let Some(pipe_right_location) = get_right_pipe(&pipes, &start_pipe) {
        let pipe_right = pipes.get(pipe_right_location).unwrap();
        let mut current_pipe = pipe_right;
        let mut entry_type = String::from("left");
        while !back_at_start {
            if let (Some(next_pipe_position), Some(next_entry_type)) =
                get_next_pipe_position(&pipes, &current_pipe, entry_type.to_string())
            {
                let next_pipe = pipes.get(next_pipe_position).unwrap(); // could be a problem
                pipes_in_loop.push(next_pipe.clone());
                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                    done = true;
                } else {
                    entry_type = next_entry_type.to_string();
                    current_pipe = next_pipe;
                }
            } else {
                // dead end
                pipes_in_loop = Vec::new();
                distance_traveled = 0;
                dbg!("dead end");
                back_at_start = true;
            }
        }
    } else {
        // restart
    }

    if !done {
        back_at_start = false;
    }

    if let Some(pipe_left_location) = get_left_pipe(&pipes, &start_pipe) {
        let pipe_left = pipes.get(pipe_left_location).unwrap();
        let mut current_pipe = pipe_left;
        let mut entry_type = String::from("right");
        while !back_at_start {
            if let (Some(next_pipe_position), Some(next_entry_type)) =
                get_next_pipe_position(&pipes, &current_pipe, entry_type.to_string())
            {
                let next_pipe = pipes.get(next_pipe_position).unwrap(); // could be a problem
                pipes_in_loop.push(next_pipe.clone());
                distance_traveled += 1;
                if next_pipe.letter == 'S' {
                    dbg!("FOUND S AGAIN ---------");
                    dbg!(distance_traveled);
                    back_at_start = true;
                    done = true;
                } else {
                    entry_type = next_entry_type.to_string();
                    current_pipe = next_pipe;
                }
            } else {
                // dead end
                pipes_in_loop = Vec::new();
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
    dbg!(pipes_in_loop.len());
    let num_of_enclosed = find_tiles_enclosed_in_loop(&pipes, &pipes_in_loop);
    dbg!(num_of_enclosed);

    // 6204 too high
    // 695 too high
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pipe_in_pipes(pipe: &Pipe, pipes: &Vec<Pipe>) -> bool {
    match pipes.iter().position(|p| p.x == pipe.x && p.y == pipe.y) {
        Some(_) => true,
        None => false,
    }
}

fn north_bounded(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> bool {
    let found_bound = false;
    let mut current_pipe = pipe;
    while !found_bound {
        if let Some(next_up) = get_up_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                return true;
            }
        } else {
            // out of bounds
            return false;
        }
    }
    return false;
}

fn south_bounded(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> bool {
    let found_bound = false;
    let mut current_pipe = pipe;
    while !found_bound {
        if let Some(next_up) = get_down_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                return true;
            }
        } else {
            // out of bounds
            return false;
        }
    }
    return false;
}

fn east_bounded(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> bool {
    let found_bound = false;
    let mut current_pipe = pipe;
    while !found_bound {
        if let Some(next_up) = get_right_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                return true;
            }
        } else {
            // out of bounds
            return false;
        }
    }
    return false;
}

fn west_bounded(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> bool {
    let found_bound = false;
    let mut current_pipe = pipe;
    while !found_bound {
        if let Some(next_up) = get_left_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                return true;
            }
        } else {
            // out of bounds
            return false;
        }
    }
    return false;
}

fn find_tiles_enclosed_in_loop(original_pipes: &Vec<Pipe>, pipes_in_loop: &Vec<Pipe>) -> u32 {
    let mut num_enclosed = 0;
    let mut new_pipes = Vec::new();
    for og_pipe in original_pipes {
        // check if it is a loop pipe
        if pipe_in_pipes(og_pipe, pipes_in_loop) {
            let new_pipe = Pipe {
                letter: og_pipe.letter.clone(),
                x: og_pipe.x.clone(),
                y: og_pipe.y.clone(),
            };
            new_pipes.push(new_pipe);
        } else {
            // check NESW
            if west_bounded(og_pipe, original_pipes, pipes_in_loop)
                && east_bounded(og_pipe, original_pipes, pipes_in_loop)
                && north_bounded(og_pipe, original_pipes, pipes_in_loop)
                && south_bounded(og_pipe, original_pipes, pipes_in_loop)
            {
                let new_pipe = Pipe {
                    letter: '?',
                    x: og_pipe.x.clone(),
                    y: og_pipe.y.clone(),
                };
                new_pipes.push(new_pipe);
                num_enclosed += 1;
            } else {
                let new_pipe = Pipe {
                    letter: '.',
                    x: og_pipe.x.clone(),
                    y: og_pipe.y.clone(),
                };
                new_pipes.push(new_pipe);
            }
        }
    }

    // let mut c_y = 0;
    // for pipe in new_pipes {
    //     if pipe.y > c_y {
    //         c_y += 1;
    //         print!("\n");
    //     }
    //     print!("{}", pipe.letter);
    // }

    let mut new_counts = 0;
    for pipe in &new_pipes {
        if pipe.letter == '?' {
            let w = west_borders_crossed(pipe, &new_pipes, pipes_in_loop);
            let e = east_borders_crossed(pipe, &new_pipes, pipes_in_loop);
            // dbg!(w);
            // dbg!(e);
            if (e + 2) % 2 != 0 {
                new_counts += 1;
            }
        }
    }
    dbg!(new_counts);

    return num_enclosed;
}

// not 669
// not 671

fn west_borders_crossed(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> i32 {
    let mut found_bound = false;
    let mut current_pipe = pipe;
    let mut num_borders_crossed = 0;
    while !found_bound {
        if let Some(next_up) = get_left_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                if current_pipe.letter == '|'
                    || current_pipe.letter == 'J'
                    || current_pipe.letter == 'L'
                    || current_pipe.letter == 'S'
                {
                    num_borders_crossed += 1;
                }
            }
        } else {
            found_bound = true;
        }
    }
    return num_borders_crossed;
}

fn east_borders_crossed(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> i32 {
    let mut found_bound = false;
    let mut current_pipe = pipe;
    let mut num_borders_crossed = 0;
    while !found_bound {
        if let Some(next_up) = get_right_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                if current_pipe.letter == '|'
                    || current_pipe.letter == 'J'
                    || current_pipe.letter == 'L'
                    || current_pipe.letter == 'S'
                {
                    num_borders_crossed += 1;
                }
            }
        } else {
            found_bound = true;
        }
    }
    return num_borders_crossed;
}

fn north_borders_crossed(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> i32 {
    let mut found_bound = false;
    let mut current_pipe = pipe;
    let mut num_borders_crossed = 0;
    while !found_bound {
        if let Some(next_up) = get_up_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                if current_pipe.letter == '-' || current_pipe.letter == 'S' {
                    num_borders_crossed += 1;
                }
            }
        } else {
            found_bound = true;
        }
    }
    return num_borders_crossed;
}

fn south_borders_crossed(pipe: &Pipe, pipes: &Vec<Pipe>, loop_pipes: &Vec<Pipe>) -> i32 {
    let mut found_bound = false;
    let mut current_pipe = pipe;
    let mut num_borders_crossed = 0;
    while !found_bound {
        if let Some(next_up) = get_down_pipe(pipes, current_pipe) {
            current_pipe = pipes.get(next_up).unwrap();
            if pipe_in_pipes(current_pipe, loop_pipes) {
                if current_pipe.letter == '-' || current_pipe.letter == 'S' {
                    num_borders_crossed += 1;
                }
            }
        } else {
            found_bound = true;
        }
    }
    return num_borders_crossed;
}
