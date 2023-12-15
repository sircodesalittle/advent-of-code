use std::collections::HashMap;
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Eq, Hash, PartialEq)]
struct SpringRow {
    record: String,
    contiguous_groups: Vec<i32>,
}

struct Memoized {
    map: HashMap<SpringRow, i64>,
}

impl Memoized {
    fn new() -> Memoized {
        Memoized {
            map: HashMap::new(),
        }
    }
}

fn count_possible_arrangements(
    condition: &str,
    groups: &[i32],
    memoization_map: &mut Memoized,
) -> i64 {
    let input = SpringRow {
        record: condition.to_string(),
        contiguous_groups: groups.to_vec(),
    };

    if memoization_map.map.contains_key(&input) {
        return *memoization_map.map.get(&input).unwrap();
    }

    if condition.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        };
    }

    let first_char = condition.chars().next().unwrap();
    let mut permutations = 0;

    if first_char == '.' {
        permutations = count_possible_arrangements(&condition[1..], groups, memoization_map);
    } else if first_char == '?' {
        permutations =
            count_possible_arrangements(&format!(".{}", &condition[1..]), groups, memoization_map)
                + count_possible_arrangements(
                    &format!("#{}", &condition[1..]),
                    groups,
                    memoization_map,
                );
    } else {
        if groups.is_empty() {
            permutations = 0;
        } else {
            let nr_damaged = groups[0] as usize;

            if nr_damaged <= condition.len()
                && condition
                    .chars()
                    .take(nr_damaged)
                    .all(|c| c == '#' || c == '?')
            {
                let new_groups = &groups[1..];
                if nr_damaged == condition.len() {
                    permutations = if new_groups.is_empty() { 1 } else { 0 };
                } else if condition.chars().nth(nr_damaged).unwrap() == '.' {
                    permutations = count_possible_arrangements(
                        &condition[nr_damaged + 1..],
                        new_groups,
                        memoization_map,
                    );
                } else if condition.chars().nth(nr_damaged).unwrap() == '?' {
                    permutations = count_possible_arrangements(
                        &format!(".{}", &condition[nr_damaged + 1..]),
                        new_groups,
                        memoization_map,
                    );
                } else {
                    permutations = 0;
                }
            } else {
                permutations = 0;
            }
        }
    }

    memoization_map.map.insert(input, permutations);
    return permutations;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut spring_rows = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ready_line) = line {
                let (springs_str, contiguous_str) = ready_line.split_once(" ").unwrap();
                let mut spring_record = springs_str.to_string();
                spring_record.push('?');
                spring_record.push_str(springs_str);
                spring_record.push('?');
                spring_record.push_str(springs_str);
                spring_record.push('?');
                spring_record.push_str(springs_str);
                spring_record.push('?');
                spring_record.push_str(springs_str);

                let original_groups = contiguous_str
                    .split(',')
                    .into_iter()
                    .map(|g| return g.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let mut groups = Vec::new();
                groups.extend(original_groups.clone());
                groups.extend(original_groups.clone());
                groups.extend(original_groups.clone());
                groups.extend(original_groups.clone());
                groups.extend(original_groups.clone());
                let spring_row = SpringRow {
                    record: spring_record,
                    contiguous_groups: groups,
                };
                spring_rows.push(spring_row);
            }
        }
    }

    let mut memoization_map = Memoized::new();
    let mut total = 0;
    for (index, spring_row) in spring_rows.iter().enumerate() {
        println!("Working on {}/{}", index + 1, spring_rows.len());
        total += count_possible_arrangements(
            &spring_row.record,
            &spring_row.contiguous_groups,
            &mut memoization_map,
        )
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
