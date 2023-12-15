use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct UniversePoint {
    x: u128,
    y: u128,
    character: char,
    galaxy_id: String,
    x_offset: u128,
    y_offset: u128,
}

impl UniversePoint {
    fn is_galaxy(&self) -> bool {
        return self.character == '#';
    }

    fn get_total_y(&self) -> i128 {
        return (self.y + self.y_offset) as i128;
    }

    fn get_total_x(&self) -> i128 {
        return (self.x + self.x_offset) as i128;
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
                        y_offset: 0,
                        x_offset: 0,
                    };
                    universe.push(universe_point);
                }
            }
        }
    }

    // print_universe(&universe);
    expand_columns(&mut universe);
    expand_rows(&mut universe);
    // print_universe(&universe);
    let mut galaxies = number_and_get_galaxies(&mut universe);
    dbg!(&galaxies.len());
    // dbg!(&galaxies);
    let total = get_shortest_path_length(&mut galaxies);
    dbg!(total);
}

fn get_max_y(universe_points: &Vec<UniversePoint>) -> u128 {
    return universe_points.iter().max_by_key(|up| up.y).unwrap().y;
}

fn get_max_x(universe_points: &Vec<UniversePoint>) -> u128 {
    return universe_points.iter().max_by_key(|up| up.x).unwrap().x;
}

fn expand_rows(universe_points: &mut Vec<UniversePoint>) {
    for row_number in 0..get_max_y(universe_points) {
        // iterate over each row
        let row_points = universe_points
            .iter()
            .filter(|up| up.y == row_number)
            .collect::<Vec<&UniversePoint>>();
        // dbg!(&row_points);
        if row_points.iter().all(|up| up.character != '#') {
            // need to add a row
            universe_points.iter_mut().for_each(|up| {
                let add_value = (row_number as u128) + 1;
                if up.y >= add_value {
                    up.y_offset += 1000000 - 1;
                }
            })
        }
    }
}

fn expand_columns(universe_points: &mut Vec<UniversePoint>) {
    for column_number in 0..get_max_x(universe_points) {
        // iterate over each row
        let column_points = universe_points
            .iter()
            .filter(|up| up.x == column_number)
            .collect::<Vec<&UniversePoint>>();
        // dbg!(&row_points);
        if column_points.iter().all(|up| up.character != '#') {
            // need to add a column
            universe_points.iter_mut().for_each(|up| {
                let add_value = (column_number as u128) + 1;
                if up.x >= add_value {
                    up.x_offset += 1000000 - 1;
                }
            })
        }
    }
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

fn get_path_length(galaxy_a: &UniversePoint, galaxy_b: &UniversePoint) -> u128 {
    let mut distance = 1;
    let x = galaxy_a.get_total_x() - galaxy_b.get_total_x();
    let y = galaxy_a.get_total_y() - galaxy_b.get_total_y();
    distance = y.abs() + x.abs();
    // dbg!(distance);
    // dbg!(galaxy_a);
    // dbg!(galaxy_b);
    // dbg!(x);
    // dbg!(y);
    return distance as u128;
}

fn get_shortest_path_length(galaxies: &mut Vec<UniversePoint>) -> u128 {
    let mut total_distance = 0;
    while let Some(temp) = galaxies.pop() {
        total_distance += galaxies
            .iter()
            .map(|g| return get_path_length(g, &temp))
            .sum::<u128>();
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
