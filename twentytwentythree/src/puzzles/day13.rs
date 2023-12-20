use std::collections::{HashMap, HashSet};

#[test]
fn test() {
    solve(String::from(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

",
    ))
}

fn find_middle(puzzle: Puzzle) {
    let mut reverse_map = HashMap::new();
    for (row_index, row_value) in puzzle.horizontals {
        reverse_map
            .entry(row_value)
            .and_modify(|r: &mut Vec<usize>| r.push(row_index))
            .or_insert(vec![row_index]);
    }
    let matches: Vec<Vec<usize>> = reverse_map.into_values().collect();
    for mut m in matches {
        m.sort();
        if m.len() > 1 {
            let first = m.first().unwrap().clone();
            m.retain(|&item| item != first || item != first + 1 || item != first - 1);
        }
        dbg!()
    }
}

struct Puzzle {
    verticals: HashMap<usize, String>,
    horizontals: HashMap<usize, String>,
}

pub fn solve(data: String) {
    let mut puzzles = Vec::new();
    let mut verticals = HashMap::new();
    let mut horizontals = HashMap::new();
    let mut line_index_offset = 0;
    for (line_index, line) in data.lines().enumerate() {
        if !line.is_empty() {
            horizontals
                .entry(line_index - line_index_offset)
                .or_insert(String::from(line));
            for (char_index, character) in line.chars().enumerate() {
                verticals
                    .entry(char_index)
                    .and_modify(|p: &mut String| p.push(character))
                    .or_insert(character.to_string());
            }
        } else {
            line_index_offset += verticals.keys().len() - 1;
            let p = Puzzle {
                verticals,
                horizontals,
            };
            puzzles.push(p);
            verticals = HashMap::new();
            horizontals = HashMap::new();
        }
    }

    for puzzle in puzzles {
        find_middle(puzzle);
    }
}
