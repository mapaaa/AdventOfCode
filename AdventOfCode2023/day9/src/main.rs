use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input() -> Vec<Vec<i64>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.to_string().parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let histories = read_input();
    let ans: i64 = 0;
    for history in histories {
        ans += predict_next(history);
    }
    println!("Answer: {}", ans);
}
