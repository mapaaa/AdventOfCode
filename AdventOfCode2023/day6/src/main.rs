use roots::{find_roots_quadratic, Roots};
use std::fs::File;
use std::io::{prelude::*, BufReader};

// Works without "Time" and "Distance"
fn read_input_puzzle_1() -> Vec<Vec<i64>> {
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

// Just give 2 numbers as input, one on each line
fn read_input_puzzle_2() -> (i64, i64) {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    (input[0], input[1])
}

fn solve_race(time: i64, distance: i64) -> i64 {
    // -x^2 + time * x - distance = 0
    let roots = find_roots_quadratic(-1.0, time as f64, -distance as f64);
    match roots {
        Roots::Two(v) => {
            let x1 = (v[1] - 0.000001).floor() as i64;
            let x2 = (v[0] + 0.000001).ceil() as i64;
            println!("Possible solutions are between {:?} and {:?}", x1, x2);
            x1 - x2 + 1
        }
        _ => 1,
    }
}

fn solve(races: Vec<Vec<i64>>) -> i64 {
    let mut ans: i64 = 1;
    let cnt_races = races[0].len();
    for i in 0..cnt_races {
        ans *= solve_race(races[0][i], races[1][i]);
    }
    ans
}

fn main() {
    let race: (i64, i64) = read_input_puzzle_2();
    println!("{:?}", race);
    println!("Answer= {:?}", solve_race(race.0, race.1));
}
