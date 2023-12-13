use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct UniversePoint {
    x: u32,
    y: u32,
    character: char,
    galaxy_id: String,
}

impl UniversePoint {
    fn is_galaxy(&self) -> bool {
        return self.character == '#';
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut universe = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for (y, line) in lines.enumerate() {
            if let Ok(ready_line) = line {
                for (x, character) in ready_line.chars().enumerate() {
                    let universe_point = UniversePoint {
                        character: character,
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        galaxy_id: "".to_string(),
                    };
                    universe.push(universe_point);
                }
            }
        }
    }

    print_universe(&universe);
    expand_columns(&mut universe);
    expand_rows(&mut universe);
    print_universe(&universe);
    let mut galaxies = number_and_get_galaxies(&mut universe);
    dbg!(&galaxies.len());
    // dbg!(&galaxies);
    let total = get_shortest_path_length(&mut galaxies);
    dbg!(total);

    // 9652524 - too high
}

fn get_max_y(universe_points: &Vec<UniversePoint>) -> u32 {
    return universe_points.iter().max_by_key(|up| up.y).unwrap().y;
}

fn get_max_x(universe_points: &Vec<UniversePoint>) -> u32 {
    return universe_points.iter().max_by_key(|up| up.x).unwrap().x;
}

fn expand_rows(universe_points: &mut Vec<UniversePoint>) {
    let mut new_universe_points = Vec::new();
    let mut added_rows = Vec::new();
    for row_number in 0..get_max_y(universe_points) {
        // iterate over each row
        let row_points = universe_points
            .iter()
            .filter(|up| up.y == row_number)
            .collect::<Vec<&UniversePoint>>();
        // dbg!(&row_points);
        if row_points.iter().all(|up| up.character != '#') {
            // need to add a row
            // dbg!("Adding a row at {}", row_number);
            added_rows.push(row_number + 1);
            new_universe_points.extend(row_points.iter().map(|up| {
                return UniversePoint {
                    x: up.x,
                    y: up.y + 1,
                    character: '.',
                    galaxy_id: String::from(""),
                };
            }));
        }
    }

    for added_row in added_rows {
        universe_points.iter_mut().for_each(|up| {
            let add_value = added_row as u32;
            if up.y >= add_value {
                let new_y: u32 = 1 + up.y;
                // println!("increasing point y: {} to {}", up.y, new_y);
                up.y = new_y.try_into().unwrap();
            }
        })
    }

    universe_points.extend(new_universe_points);
}

fn expand_columns(universe_points: &mut Vec<UniversePoint>) {
    let mut new_universe_points = Vec::new();
    let mut added_columns = Vec::new();
    let mut num_added = 0;
    for column_number in 0..get_max_x(universe_points) {
        // iterate over each row
        let column_points = universe_points
            .iter()
            .filter(|up| up.x == column_number)
            .collect::<Vec<&UniversePoint>>();
        // dbg!(&row_points);
        if column_points.iter().all(|up| up.character != '#') {
            // need to add a row
            // println!("Found column of all . : at column index {}", column_number);
            // println!("Adding a row at {}", column_number + 1);
            added_columns.push(column_number + 1 + num_added);
            new_universe_points.extend(column_points.iter().map(|up| {
                // println!("adding point at x: {}", up.x + 1 + num_added);
                return UniversePoint {
                    x: up.x + 1 + num_added,
                    y: up.y,
                    character: '.',
                    galaxy_id: "".to_string(),
                };
            }));

            num_added += 1;
        }
    }

    for added_column in added_columns {
        universe_points.iter_mut().for_each(|up| {
            let add_value = added_column as u32;
            if up.x >= add_value {
                let new_x: u32 = 1 + up.x;
                // println!("increasing point x: {} to {}", up.x, new_x);
                up.x = new_x.try_into().unwrap();
            }
        })
    }

    universe_points.extend(new_universe_points);
}

fn print_universe(universe: &Vec<UniversePoint>) {
    for row in 0..(get_max_y(universe) + 1) {
        for column in 0..(get_max_x(universe) + 1) {
            let position = universe
                .iter()
                .position(|up| up.x == column && up.y == row)
                .unwrap();
            let point_to_print = universe.get(position).unwrap();
            if point_to_print.is_galaxy() {
                print!("{}", point_to_print.character);
            } else {
                print!("{}", point_to_print.character);
            }
        }
        print!("\n");
    }
}

fn number_and_get_galaxies(universe: &mut Vec<UniversePoint>) -> Vec<UniversePoint> {
    let mut galaxy_id = 1;
    let mut galaxies = Vec::new();
    for point in universe {
        if point.is_galaxy() {
            point.galaxy_id = galaxy_id.to_string();
            galaxy_id += 1;
            galaxies.push(point.clone());
        }
    }
    return galaxies;
}

fn get_path_length(galaxy_a: &UniversePoint, galaxy_b: &UniversePoint) -> u32 {
    let mut distance = 1;
    let y = (galaxy_a.y as i32) - (galaxy_b.y as i32).abs();
    let x = (galaxy_a.x as i32) - (galaxy_b.x as i32).abs();
    distance = y.abs() + x.abs();
    // dbg!(distance);
    // dbg!(galaxy_a);
    // dbg!(galaxy_b);
    // dbg!(x);
    // dbg!(y);
    return distance as u32;
}

fn get_shortest_path_length(galaxies: &mut Vec<UniversePoint>) -> u32 {
    let mut total_distance = 0;
    while let Some(temp) = galaxies.pop() {
        total_distance += galaxies
            .iter()
            .map(|g| return get_path_length(g, &temp))
            .sum::<u32>();
    }
    return total_distance;
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
