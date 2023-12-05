use std::cmp::max;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn check_state(mut i: usize, index: usize, words: &Vec<&str>) -> (i32, i32, i32) {
    println!("Processing:");
    for x in i..(index + 1) {
        println!("{:?}", words[x]);
    }
    println!("---");
    let mut s_red: i32 = 0;
    let mut s_green: i32 = 0;
    let mut s_blue: i32 = 0;
    while i <= index {
        let quantity: i32 = words[i].parse().expect("Please type a number");
        let color: String = words[i + 1].to_string();
        println!("Am {:?} cu cantitatea {:?}", color, quantity);
        if color == "red" || color == "red;" {
            s_red += quantity;
        } else if color == "green" || color == "green;" {
            s_green += quantity;
        } else if color == "blue" || color == "blue;" {
            s_blue += quantity;
        }
        i += 3;
    }
    println!("----");
    return (s_red, s_green, s_blue);
}

fn solve(line: String) -> (i32, i32, i32) {
    let words: Vec<&str> = line.split([' ', ':', ','].as_ref()).collect();
    let len = words[1].len();
    let game_number: i32 = (&words[1][0..len]).parse().expect("Please type a number!");
    println!("Game number is {:?}", game_number);
    let mut last_index: usize = 3;
    let mut max_red: i32 = -1;
    let mut max_green: i32 = -1;
    let mut max_blue: i32 = -1;
    for index in 3..words.len() {
        if words[index].chars().last() == Some(';') || index == words.len() - 1 {
            let (min_red, mid_green, min_blue) =
                check_state(last_index.clone(), index.clone(), &words);
            max_red = max(min_red, max_red);
            max_green = max(mid_green, max_green);
            max_blue = max(min_blue, max_blue);
            last_index = index + 1;
        }
    }
    return (max_red, max_green, max_blue);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let ans: i32 = reader
        .lines()
        .map(|line| {
            let (red, green, blue) = solve(line.unwrap());
            red * green * blue
        })
        .sum();

    println!("Ans: {:?}", ans);
}
