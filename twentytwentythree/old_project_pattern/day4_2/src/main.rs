use std::collections::HashMap;
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
    card_number: u32,
}

struct WinningCard {
    card: Card,
    copy_cards: Vec<u32>,
}

impl Card {
    fn get_winning_card(&self) -> WinningCard {
        let mut my_winning_numbers = Vec::new();
        for my_number in &self.my_numbers {
            if self.winning_numbers.contains(my_number) {
                my_winning_numbers.push(my_number);
            }
        }
        let mut copy_cards = Vec::new();
        let mut copy_card_index = self.card_number + 1;
        if my_winning_numbers.len() > 0 {
            for _winning_num in my_winning_numbers {
                copy_cards.push(copy_card_index);
                copy_card_index += 1;
            }
        }
        return WinningCard {
            card: (self.clone()),
            copy_cards: (copy_cards),
        };
    }
}

fn get_list_from_str_list(str_list_of_numbers: &str) -> Vec<u32> {
    // pass in something like "32 52 52 12 53"
    let mut num_list = Vec::new();
    for str_num in str_list_of_numbers.split(" ").into_iter() {
        if str_num != "" {
            num_list.push(str_num.trim().parse::<u32>().unwrap());
        };
    }
    return num_list;
}

fn get_card_from_line(line: String) -> Card {
    // Card number
    let (card_number_str, numbers) = line.split_once(": ").unwrap();
    let (_, card_number_unclean) = card_number_str.split_once(" ").unwrap();
    let card_number_unparsed = card_number_unclean.trim_start();
    let card_number = card_number_unparsed.parse::<u32>().unwrap();

    let (winning_numbers_unclean, my_numbers_unclean) = numbers.split_once(" | ").unwrap();
    // winning numbers
    let winning_numbers = get_list_from_str_list(winning_numbers_unclean);

    // my numbers
    let my_numbers = get_list_from_str_list(my_numbers_unclean);

    let new_card = Card {
        winning_numbers: winning_numbers,
        my_numbers: my_numbers,
        card_number: card_number,
    };
    return new_card;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut cards = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ready_line) = line {
                let card = get_card_from_line(ready_line);
                cards.push(card);
            }
        }
    }

    let mut winning_cards = Vec::new();
    for card in cards {
        winning_cards.push(card.get_winning_card());
    }

    let mut total = 0;
    let mut value_map = HashMap::new();
    let mut other_map = value_map.clone();
    winning_cards.reverse();
    for winning_card in winning_cards {
        let current_value = value_map.entry(winning_card.card.card_number).or_insert(1);
        for copy_card_number in winning_card.copy_cards {
            *current_value += other_map.get(&copy_card_number).unwrap()
        }
        other_map = value_map.clone();
        // total += winning_card.get_total_cards();
    }
    dbg!(value_map);

    for (_game_number, value) in &other_map {
        total += value;
    }

    // too low: 1058
    println!("Total: {}", total);
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
