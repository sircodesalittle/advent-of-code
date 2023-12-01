use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current_floor = 0;
    let mut characters_processed = 0;
    let mut entered_basement = false;
    let contents_itr = contents.char_indices();
    for character in contents_itr {
        characters_processed += 1;
        let directive = character.1;
        if directive == ')' {
            current_floor -= 1;
        }
        if directive == '(' {
            current_floor += 1;
        }
        if current_floor == (-1) && !entered_basement {
            println!("in the basement first: {}", characters_processed);
            entered_basement = true;
        }
    }
    println!("Floor: {}", current_floor)
}
