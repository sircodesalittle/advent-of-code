use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Band {
    source_start: u128,
    source_end: u128,
    destination_start: u128,
}

impl Band {
    fn in_band(&self, value: u128) -> bool {
        if value >= self.source_start && value <= self.source_end {
            return true;
        }
        return false;
    }
}
struct SpecialMap {
    bands: Vec<Band>,
}

impl SpecialMap {
    fn lookup(&self, value: u128) -> u128 {
        let mut found = false;
        let mut found_value = value;
        for band in &self.bands {
            if !found && band.in_band(value) {
                let offset = value - band.source_start;
                found_value = band.destination_start + offset;
                found = true;
            }
        }
        return found_value;
    }

    fn add_band(&mut self, band: Band) {
        self.bands.push(band);
    }
}

fn process_map_line(map_line: &str, current: &mut SpecialMap) {
    let map_line_split: Vec<&str> = map_line.split(" ").collect();
    let destination = map_line_split[0].parse::<u128>().unwrap();
    let source = map_line_split[1].parse::<u128>().unwrap();
    let range_length = map_line_split[2].parse::<u128>().unwrap();
    let new_band = Band {
        source_start: source,
        source_end: source + range_length,
        destination_start: destination,
    };
    current.add_band(new_band);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut seeds = Vec::new();
    let mut seed_soil = SpecialMap { bands: Vec::new() };
    let mut soil_fertilizer = SpecialMap { bands: Vec::new() };
    let mut fertilizer_water = SpecialMap { bands: Vec::new() };
    let mut water_light = SpecialMap { bands: Vec::new() };
    let mut light_temp = SpecialMap { bands: Vec::new() };
    let mut temp_humidity = SpecialMap { bands: Vec::new() };
    let mut humidity_location = SpecialMap { bands: Vec::new() };
    let mut current_map = &mut seed_soil;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ready_line) = line {
                if ready_line != "" {
                    if ready_line.contains("seeds: ") {
                        let (_, seeds_str) = ready_line.split_once(": ").unwrap();
                        for seed in seeds_str.split(" ") {
                            seeds.push(seed.parse::<u128>().unwrap())
                        }
                    } else if ready_line.contains("seed-to-soil map:") {
                        // this is the default
                        current_map = &mut seed_soil;
                    } else if ready_line.contains("soil-to-fertilizer map:") {
                        current_map = &mut soil_fertilizer;
                    } else if ready_line.contains("fertilizer-to-water map:") {
                        current_map = &mut fertilizer_water;
                    } else if ready_line.contains("water-to-light map:") {
                        current_map = &mut water_light;
                    } else if ready_line.contains("light-to-temperature map:") {
                        current_map = &mut light_temp;
                    } else if ready_line.contains("temperature-to-humidity map:") {
                        current_map = &mut temp_humidity;
                    } else if ready_line.contains("humidity-to-location map:") {
                        current_map = &mut humidity_location;
                    } else {
                        process_map_line(&ready_line, current_map);
                    }
                } else {
                }
            }
        }
    }

    let mut lowest_location = u128::MAX;
    for seed in seeds {
        let soil = seed_soil.lookup(seed);
        let fertilizer = soil_fertilizer.lookup(soil);
        let water = fertilizer_water.lookup(fertilizer);
        let light = water_light.lookup(water);
        let temp = light_temp.lookup(light);
        let humidity = temp_humidity.lookup(temp);
        let location = humidity_location.lookup(humidity);
        if location < lowest_location {
            lowest_location = location;
        }
    }
    println!("Lowest location: {}", lowest_location);
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
