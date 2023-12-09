use std::collections::HashMap;
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut node_map = HashMap::new();
    let mut instructions = None;
    let mut line_number = 1;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ready_line) = line {
                if line_number == 1 {
                    let instr = ready_line.to_owned();
                    instructions = Some(instr);
                } else if !ready_line.is_empty() {
                    let mut node_key_and_instructions =
                        ready_line.split(" = ").map(|s| s.to_string());
                    let node_key = node_key_and_instructions.next().unwrap();
                    let instructions = node_key_and_instructions.next().unwrap();
                    let mut clean_instructions = instructions
                        .strip_prefix("(")
                        .map(|s| s.to_string())
                        .unwrap();
                    clean_instructions = clean_instructions
                        .strip_suffix(")")
                        .map(|s| s.to_string())
                        .unwrap();
                    let mut left_right = clean_instructions.split(", ").map(|s| s.to_string());
                    let left = left_right.next().unwrap();
                    let right = left_right.next().unwrap();
                    node_map.insert(node_key, [left, right]);
                }
            }
            line_number += 1;
        }
    }

    // dbg!(&node_map);
    let mut traversal_count = 0;
    let mut current_node = String::from("AAA");
    let mut found_end = false;
    if let Some(instructions) = instructions {
        while !found_end {
            for instruction in instructions.chars() {
                if !found_end {
                    // println!("{}", current_node);
                    let node_instructions = node_map.get(&current_node).unwrap();
                    let left_instruction = node_instructions.get(0).unwrap();
                    let right_instruction = node_instructions.get(1).unwrap();
                    if instruction == 'R' {
                        current_node = right_instruction.clone();
                    } else if instruction == 'L' {
                        current_node = left_instruction.clone();
                    }
                    // println!("moving to: {}", current_node);
                    traversal_count += 1;
                    if current_node == "ZZZ" {
                        found_end = true;
                    }
                } else {
                    println!("{}", traversal_count);
                }
            }
        }
    }

    dbg!(traversal_count);
    // 16 is not the right answer
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
