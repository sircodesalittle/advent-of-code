use std::collections::HashSet;
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
struct SpringRow {
    record: String,
    contiguous_groups: Vec<u8>,
}

fn get_combinations(number_of_unknown: u8) -> Vec<String> {
    // credits: https://stackoverflow.com/questions/67746583/get-all-combinations-of-a-vector-of-n-chars
    let characters = vec![".", "#"];
    let n = number_of_unknown;

    let combinations: Vec<_> = (2..n).fold(
        characters
            .iter()
            .map(|c| characters.iter().map(move |&d| d.to_owned() + *c))
            .flatten()
            .collect(),
        |acc, _| {
            acc.into_iter()
                .map(|c| characters.iter().map(move |&d| d.to_owned() + &*c))
                .flatten()
                .collect()
        },
    );
    return combinations;
}

impl SpringRow {
    fn get_different_arrangements(&self) -> HashSet<String> {
        let mut possible_arrangements = HashSet::new();
        let num_questions = self.record.matches("?").collect::<Vec<&str>>().len();
        for combination in get_combinations(num_questions as u8) {
            let mut new_arrangement = self.record.clone();
            for possible in combination.chars() {
                let new_new = new_arrangement.replacen("?", possible.to_string().as_str(), 1);
                new_arrangement = new_new;
            }
            possible_arrangements.insert(new_arrangement);
        }
        return possible_arrangements;
    }

    fn get_num_different_arrangements(&self) -> u32 {
        let mut arrangements = 0;
        let arrangements_to_test = self.get_different_arrangements();
        for possible_arrangement in arrangements_to_test {
            if self.is_valid_arrangement(&possible_arrangement) {
                arrangements += 1;
            }
        }
        return arrangements;
    }

    fn is_valid_arrangement(&self, possible_arrangement: &String) -> bool {
        let mut arrangements = self.contiguous_groups.clone();
        arrangements.reverse();
        let mut chars: Vec<char> = possible_arrangement.chars().collect();
        chars.dedup_by(|a, b| *a == '.' && *b == '.');
        let removed_dup_periods: String = chars.iter().collect();
        let groups_to_validate: Vec<&str> = removed_dup_periods
            .split(".")
            .filter(|x| *x != "")
            .collect();

        if groups_to_validate.len() != arrangements.len() {
            return false;
        }

        let mut is_valid = true;
        let mut group_index = 0;
        while let Some(grouping) = arrangements.pop() {
            if grouping as usize != groups_to_validate[group_index].len() {
                is_valid = false;
            }
            group_index += 1;
        }
        return is_valid;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut spring_rows = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ready_line) = line {
                let (springs_str, contiguous_str) = ready_line.split_once(" ").unwrap();
                let spring_row = SpringRow {
                    record: springs_str.to_string(),
                    contiguous_groups: contiguous_str
                        .split(',')
                        .into_iter()
                        .map(|g| return g.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>(),
                };
                spring_rows.push(spring_row);
            }
        }
    }

    let mut total = 0;
    for (index, spring_row) in spring_rows.iter().enumerate() {
        println!("Working on {}/{}", index + 1, spring_rows.len());
        total += spring_row.get_num_different_arrangements();
    }
    dbg!(total);
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
