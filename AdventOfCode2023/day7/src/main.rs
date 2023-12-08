use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input_puzzle_1() -> Vec<(String, String)> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect::<Vec<Vec<String>>>()
        .iter()
        .map(|l| (l[0].clone(), l[1].clone()))
        .collect()
}

fn card_strength(card: &char) -> usize {
    match card {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        '9' => 7,
        '8' => 6,
        '7' => 5,
        '6' => 4,
        '5' => 3,
        '4' => 2,
        '3' => 1,
        '2' => 0,
        _ => {
            println!("Not a valid card");
            0
        }
    }
}

fn card_strength_with_joker(card: &char) -> usize {
    match card {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        _ => {
            println!("Not a valid card");
            0
        }
    }
}

fn hand_type(hand: &String) -> i32 {
    let cards: Vec<char> = hand.chars().collect();

    let mut counts: [usize; 13] = [0; 13];
    for &card in &cards {
        let card_str = card_strength(&card);
        counts[card_str] += 1;
    }

    if counts.iter().any(|&count| count == 5) {
        return 7; // Five of a kind
    } else if counts.iter().any(|&count| count == 4) {
        return 6; // Four of a kind
    } else if counts.iter().any(|&count| count == 3) && counts.iter().any(|&count| count == 2) {
        return 5; // Full house
    } else if counts.iter().any(|&count| count == 3) {
        return 4; // Three of a kind
    } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
        return 3; // Two pair
    } else if counts.iter().filter(|&&count| count == 2).count() == 1 {
        return 2; // One pair
    } else {
        return 1; // High card
    }
}

fn hand_type_with_joker(hand: &String) -> i32 {
    let mut counts = HashMap::new();
    let mut joker_count = 0;
    let cards: Vec<char> = hand.chars().collect();
    for &card in &cards {
        if card == 'J' {
            joker_count += 1;
        } else {
            *counts.entry(card).or_insert(0) += 1;
        }
    }

    for _ in 0..joker_count {
        if let Some((&card, &count)) = counts.iter().max_by_key(|&(_, &count)| count) {
            *counts.entry(card).or_insert(0) += 1;
        } else {
            counts.insert('A', 1);
        }
    }

    // Determine the hand type based on counts
    let mut sorted_counts = counts.values().collect::<Vec<_>>();
    sorted_counts.sort();
    sorted_counts.reverse();
    match sorted_counts.as_slice() {
        [5] => return 7,
        [4, ..] => return 6,
        [3, 2] => return 5,
        [3, ..] => return 4,
        [2, 2, ..] => return 3,
        [2, ..] => return 2,
        _ => return 1,
    }
}

fn compare_hands(x: &String, y: &String) -> Ordering {
    let cards_x: Vec<char> = x.chars().collect();
    let cards_y: Vec<char> = y.chars().collect();

    for i in 0..5 {
        let card_x = cards_x[i];
        let card_y = cards_y[i];

        if card_x != card_y {
            let card_x_str = card_strength_with_joker(&card_x);
            let card_y_str = card_strength_with_joker(&card_y);
            return card_x_str.cmp(&card_y_str);
        }
    }

    Ordering::Equal
}

fn comp_rank(x: &(String, String), y: &(String, String)) -> Ordering {
    let type_x = hand_type_with_joker(&x.0);
    let type_y = hand_type_with_joker(&y.0);
    if type_x == type_y {
        compare_hands(&x.0, &y.0)
    } else {
        type_x.cmp(&type_y)
    }
}

fn main() {
    let mut camel_hands = read_input_puzzle_1();
    println!("{:?}", camel_hands);
    camel_hands.sort_by(|x, y| comp_rank(x, y));
    println!("Sorted hands are: {:?}", camel_hands);
    let mut ans: i32 = 0;
    for (i, hand) in camel_hands.into_iter().enumerate() {
        ans += ((i + 1) as i32) * hand.1.parse::<i32>().expect("Bid is not a number!");
    }
    println!("Answer is {ans}");
}
