use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input_puzzle() -> (String, HashMap<String, (String, String)>) {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let reader: Vec<Vec<String>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .replace('(', "")
                .replace(')', "")
                .replace(',', "")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
    let steps: String = reader[0][0].clone();
    let map: HashMap<String, (String, String)> = reader[2..]
        .iter()
        .map(|l| (l[0].clone(), (l[2].clone(), l[3].clone())))
        .collect();
    (steps, map)
}

fn steps_to_reach(
    steps: String,
    map: HashMap<String, (String, String)>,
    start: String,
    end: String,
) -> usize {
    let mut steps_taken = 0;
    let mut current_location: String = start.clone();
    let mut index = 0;
    let char_steps: Vec<char> = steps.chars().collect();
    loop {
        /*  if current_location == end {
            break;
        }*/
        if current_location.ends_with('Z') {
            println!("Reached Z for {:?}", start);
            break;
        }
        let (left_location, right_location) = map.get(&current_location).unwrap();
        match char_steps[index] {
            'L' => current_location = (*left_location).clone(),
            'R' => current_location = (*right_location).clone(),
            _ => {
                println!("Not a valid step");
                break;
            }
        }
        steps_taken += 1;
        index += 1;
        if index == char_steps.len() {
            index = 0;
        }
    }
    steps_taken
}

fn get_start_locations(map: &HashMap<String, (String, String)>) -> Vec<String> {
    map.iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| k.clone())
        .collect()
}

fn steps_from_all_starts(steps: String, map: HashMap<String, (String, String)>) -> usize {
    let mut start_locations: Vec<String> = get_start_locations(&map);
    start_locations.sort();
    println!("Start locations: {:?}", start_locations);

    let mut ans = 1;
    for l in start_locations {
        ans = lcm(
            ans,
            steps_to_reach(
                steps.clone(),
                map.clone(),
                l.clone(),
                "ZZZ".to_string().clone(),
            ),
        );
    }
    ans
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn main() {
    let (lr_steps, map) = read_input_puzzle();
    println!("Steps: {lr_steps}");
    println!("Map: {:?}", map);
    read_input_puzzle();
    for (k, (v1, v2)) in map.iter() {
        if k == v1 || k == v2 {
            println!("{}: {}, {}", k, v1, v2);
        }
    }
    println!("Stop");

    //Puzzle 1: let ans = steps_to_reach(lr_steps.clone(), map.clone(), "ZZZ".to_string().clone());
    let ans = steps_from_all_starts(lr_steps.clone(), map.clone()); // Puzzle 2
    println!("Answer: {ans}");
}
