use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if let Ok(lines) = read_lines(file_path) {
        let mut total: i32 = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                println!("{}", word);
                let word_number = get_word_number(word);
                println!("{}", word_number);
                total += word_number;
            }
        }
        println!("Total: {}", total)
    }
}

fn make_word_numbers_numbers(word: String) -> String {
    return word
        .replace("twone", "twoone")
        .replace("nineight", "nineeight")
        .replace("eightwo", "eighttwo")
        .replace("eighthree", "eightthree")
        .replace("oneight", "oneeight")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
}

fn get_word_number(unclean_word: String) -> i32 {
    // not 56002 (too high)
    // not 55680 (too low)
    let word = make_word_numbers_numbers(unclean_word);
    let mut first_num: char;
    for c in word.chars() {
        if c.is_digit(10) {
            first_num = c;
            for c_2 in word.chars().rev() {
                if c_2.is_digit(10) {
                    return format!("{}{}", first_num, c_2).parse().unwrap();
                }
            }
        }
    }
    return 0;
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
