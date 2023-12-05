use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const WINNING_SET_START_POS: usize = 2;

fn get_winning_set(card: &Vec<String>) -> HashSet<i32> {
    let sep_pos: usize = card.iter().position(|x| x == "|").unwrap();
    let mut set: HashSet<i32> = HashSet::new();
    for i in WINNING_SET_START_POS..sep_pos {
        set.insert(card[i].parse::<i32>().unwrap());
    }
    set
}

fn get_elfs_set(card: &Vec<String>) -> HashSet<i32> {
    let sep_pos: usize = card.iter().position(|x| x == "|").unwrap();
    let start_pos: usize = sep_pos + 1;
    let mut set: HashSet<i32> = HashSet::new();
    for i in start_pos..card.len() {
        set.insert(card[i].parse::<i32>().unwrap());
    }
    set
}

fn get_score_and_part_two(card: &Vec<String>) -> (u32, usize) {
    let winning: HashSet<i32> = get_winning_set(&card);
    let elf: HashSet<i32> = get_elfs_set(&card);

    let mut cnt: u32 = 0;
    for x in elf {
        if winning.contains(&x) {
            cnt += 1;
        }
    }
    if cnt > 0 {
        (u32::pow(2, cnt - 1), (cnt as usize))
    } else {
        (cnt, (cnt as usize))
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let input: Vec<Vec<String>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
    println!("{:?}", input);
    let mut sum: u32 = 0;
    let mut total_scratch_cards: i32 = 0;
    let mut dif: Vec<i32> = vec![0; input.len()];
    let mut last: i32 = 0;
    for (i, card) in input.iter().enumerate() {
        let current: i32 = (last + dif[i]);
        let (partial_score, total_matches) = get_score_and_part_two(&card);
        sum += partial_score;
        total_scratch_cards += 1 + current;
        let mut lim: usize = i + total_matches;
        if lim > input.len() {
            lim = input.len();
        }
        println!("Card {:?} total_mathces = {:?}", i + 1, total_matches);
        if i + 1 < input.len() {
            dif[i + 1] += (current + 1);
        }
        if lim + 1 != input.len() {
            dif[lim + 1] -= (current + 1);
        }
        /*for j in (i + 1)..lim {
            println!("Card {:?} won {:?}", i + 1, j + 1);
            won_scratched_cards[j] += won_scratched_cards[i] + 1;
        }*/
        last = current;
    }
    println!("{:?}", sum);
    println!("{:?}", total_scratch_cards); // 9997537
}
