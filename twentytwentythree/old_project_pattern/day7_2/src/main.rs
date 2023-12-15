use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn value(&self) -> u32 {
        match *self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Joker => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
    let mut copy_cards = cards.clone();
    let copy_cards2 = cards.clone();
    let mut hand_type = HandType::HighCard;
    copy_cards.sort();
    copy_cards.reverse();
    let cards_as_set: HashSet<Card> = HashSet::from_iter(cards.clone());
    hand_type = match cards_as_set.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            let mut hand_t = HandType::FullHouse;
            let mut seen_three = false;
            let mut seen_two = false;
            let mut card_counts = HashMap::new();
            for card in copy_cards2 {
                *card_counts.entry(card).or_insert(0) += 1;
            }
            for (_card, card_count) in card_counts {
                if card_count == 4 {
                    hand_t = HandType::FourOfAKind;
                }
                if card_count == 3 {
                    seen_three = true;
                }
                if card_count == 2 {
                    seen_two = true;
                }
            }
            if seen_three && seen_two {
                hand_t = HandType::FullHouse
            }
            hand_t
        }
        3 => {
            let mut hand_t = HandType::TwoPair;
            let mut card_counts = HashMap::new();
            for card in copy_cards2 {
                *card_counts.entry(card).or_insert(0) += 1;
            }
            for (_card, card_count) in card_counts {
                if card_count == 3 {
                    hand_t = HandType::ThreeOfAKind;
                }
            }
            hand_t
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => HandType::HighCard,
    };
    return hand_type;
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: u32) -> Self {
        let mut copy_cards = cards.clone();
        // copy_cards.sort();
        // copy_cards.reverse();
        // let highest_value = copy_cards.get(0).unwrap();
        let mut card_counts = HashMap::new();
        for card in copy_cards {
            if card != Card::Joker {
                *card_counts.entry(card).or_insert(0) += 1;
            }
        }
        let mut highest_value = Card::Ace;
        if let Some(highest_value_from_map) = card_counts.iter().max_by_key(|entry| entry.1) {
            highest_value = highest_value_from_map.0.clone();
        } else {
            let mut copy_cards = cards.clone();
            copy_cards.sort();
            copy_cards.reverse();
            highest_value = copy_cards.get(0).unwrap().clone();
        }
        let mut new_cards = Vec::new();
        for mut card in cards.clone() {
            if card == Card::Joker {
                card = highest_value.clone();
            }
            new_cards.push(card);
        }
        let hand_type = calculate_hand_type(&new_cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut hands = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ready_line) = line {
                hands.push(line_to_hand(&ready_line));
            }
        }
    }

    hands.sort_by(|h1, h2| {
        if h1.hand_type == h2.hand_type {
            let mut found_winner = false;
            let mut ordering = Ordering::Greater;
            for hand_index in 0..5 {
                if found_winner != true {
                    if h1.cards.get(hand_index) < h2.cards.get(hand_index) {
                        found_winner = true;
                        ordering = Ordering::Less;
                    }
                    if h1.cards.get(hand_index) > h2.cards.get(hand_index) {
                        found_winner = true;
                        ordering = Ordering::Greater;
                    }
                }
            }
            return ordering;
        }
        return h1.hand_type.cmp(&h2.hand_type);
    });
    let mut total = 0;
    let mut rank = 1;
    for hand in hands {
        dbg!(&hand.cards, &hand.bid);
        let winnings = hand.bid * rank;
        total += winnings;
        rank += 1;
    }
    dbg!(total);
    // 251206521 too low
    // 251806792
}

fn line_to_hand(line: &String) -> Hand {
    let (cards_string, bid_string) = line.split_once(" ").unwrap();
    let bid = bid_string.parse::<u32>().unwrap();
    let mut cards = Vec::new();
    for card_char in cards_string.chars() {
        let card = match card_char {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!(),
        };
        cards.push(card);
    }
    return Hand::new(cards, bid);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
