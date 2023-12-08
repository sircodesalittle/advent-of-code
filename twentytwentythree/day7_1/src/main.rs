use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
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
            Card::Jack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

fn get_tie_break_value(cards: &Vec<Card>) -> u32 {
    let mut value = 0;
    for card in cards {
        value += card.value()
    }
    return value;
}

fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
    let mut copy_cards = cards.clone();
    let mut copy_cards2 = cards.clone();
    let mut hand_type = HandType::HighCard;
    let mut tie_break_value: u32 = get_tie_break_value(&copy_cards);
    copy_cards.sort();
    copy_cards.reverse();
    // Five of a kind   7 (* card number)  7*9 = 45, beats 5*7=35
    // four of a kind   6 (* card number)  4*
    // full house       5 (* card_1)
    // three of a kind  4 (* card number)
    // two pair         3 (* card number)
    // one pair         2 (* card number)
    // high card        1 (* card number)
    let cards_as_set: HashSet<Card> = HashSet::from_iter(cards.clone());
    hand_type = match cards_as_set.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            // four of a kind
            // full house
            let mut hand_t = HandType::FullHouse;
            let mut seen_three = false;
            let mut value_with_three = 0;
            let mut seen_two = false;
            let mut card_counts = HashMap::new();
            for card in copy_cards2 {
                *card_counts.entry(card).or_insert(0) += 1;
            }
            for (card, card_count) in card_counts {
                if card_count == 4 {
                    hand_t = HandType::FourOfAKind;
                }
                if card_count == 3 {
                    seen_three = true;
                    value_with_three = card.value();
                }
                if card_count == 2 {
                    seen_two = true;
                }
            }
            if seen_three && seen_two {
                hand_t = HandType::FullHouse
            }
            tie_break_value = value_with_three;
            hand_t
        }
        3 => {
            let mut hand_t = HandType::TwoPair;
            let mut card_counts = HashMap::new();
            let mut card_value1 = 0;
            for card in copy_cards2 {
                *card_counts.entry(card).or_insert(0) += 1;
            }
            for (card, card_count) in card_counts {
                if card_count == 2 {
                    tie_break_value = card.value();
                    if card_value1 == 0 {
                        card_value1 = card.value();
                    } else {
                        tie_break_value = (card_value1 * 2) + (card.value() * 2);
                    }
                }
                if card_count == 3 {
                    tie_break_value = card.value();
                    hand_t = HandType::ThreeOfAKind;
                }
            }
            hand_t
        }
        4 => {
            let mut card_counts = HashMap::new();
            for card in copy_cards2 {
                *card_counts.entry(card).or_insert(0) += 1;
            }
            for (card, card_count) in card_counts {
                if card_count == 2 {
                    tie_break_value = card.value();
                }
            }
            HandType::OnePair
        }
        5 => {
            tie_break_value = copy_cards.get(0).unwrap().value();
            HandType::HighCard
        }
        _ => HandType::HighCard,
    };
    return hand_type;
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: u32) -> Self {
        let hand_type = calculate_hand_type(&cards);
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
        dbg!(&hand);
        let winnings = hand.bid * rank;
        dbg!(hand.bid);
        dbg!(rank);
        dbg!(winnings);
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
            'J' => Card::Jack,
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
