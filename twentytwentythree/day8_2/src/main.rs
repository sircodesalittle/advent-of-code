use std::collections::HashMap;
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn all_found_end(map: &HashMap<String, (bool, i32)>) -> bool {
    return map.values().into_iter().all(|x| x.0);
}

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

    let mut current_nodes: HashMap<String, (bool, i32)> = HashMap::new();
    for node in node_map.keys() {
        if node.ends_with('A') {
            current_nodes.insert(node.to_string(), (false, 0));
        }
    }

    dbg!(&current_nodes);
    let mut total_count = 0;
    let mut found_end = all_found_end(&current_nodes);
    if let Some(instructions) = instructions {
        while !found_end {
            for instruction in instructions.chars() {
                let mut keys_to_remove = Vec::new();
                let mut new_current_nodes = HashMap::new();
                for current_node in current_nodes.keys() {
                    keys_to_remove.push(String::from(current_node));
                    let (found_end_for_node, traversal_count_for_node) =
                        current_nodes.get(current_node).unwrap();

                    let mut new_node = None;
                    let mut new_found_end_for_node = found_end_for_node.clone();
                    let mut new_traversal_count_for_node = traversal_count_for_node.clone();
                    // println!("{}", current_node);
                    let node_instructions = node_map.get(current_node).unwrap();
                    let left_instruction = node_instructions.get(0).unwrap();
                    let right_instruction = node_instructions.get(1).unwrap();
                    if instruction == 'R' {
                        new_node = Some(String::from(right_instruction));
                    } else if instruction == 'L' {
                        new_node = Some(String::from(left_instruction));
                    }
                    let new_node_real = new_node.unwrap();
                    // println!("moving to: {}", current_node);
                    new_traversal_count_for_node += 1;
                    if new_node_real.ends_with('Z') {
                        new_found_end_for_node = true;
                    } else {
                        new_found_end_for_node = false;
                    }
                    new_current_nodes.insert(
                        new_node_real,
                        (new_found_end_for_node, new_traversal_count_for_node),
                    );
                }
                total_count += 1;
                for key_to_remove in keys_to_remove {
                    current_nodes.remove(&key_to_remove);
                }
                current_nodes.extend(new_current_nodes);
                found_end = all_found_end(&current_nodes);
            }
            dbg!(&current_nodes);
        }
    }

    dbg!(total_count);
    // 16 is not the right answer
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
