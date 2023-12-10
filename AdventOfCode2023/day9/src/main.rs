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

fn compute_pascal_triangle(n: i64) -> Vec<Vec<i64>> {
    let mut pascal_triangle: Vec<Vec<i64>> = Vec::new();
    for i in 0..n {
        let mut row: Vec<i64> = Vec::new();
        for j in 0..(i + 1) {
            if j == 0 || j == i {
                row.push(1);
            } else {
                row.push(pascal_triangle[(i - 1) as usize][(j - 1) as usize] + pascal_triangle[(i - 1) as usize][j as usize]);
            }
        }
        pascal_triangle.push(row);
    }
    pascal_triangle
}

fn predict_next(history: &Vec<i64>, pascal_triangle: &Vec<Vec<i64>>) -> i64{
    let mut ans: i64 = 0;
    let n = history.len();
    for i in 0..history.len() {
        for j in 0..(i + 1) {
            let number = (if j % 2 == 0 {1} else {-1}) * history[n - j - 1] * pascal_triangle[i][j];
            ans += number;
        }
    }
    println!("{} {}", history.len(), ans);
    ans
}

fn predict_prev(history: &Vec<i64>, pascal_triangle: &Vec<Vec<i64>>) -> i64{
    let mut ans: i64 = 0;
    let n = history.len();
    for i in 0..history.len() {
        for j in 0..(i + 1) {
            let number = (if j % 2 == 0 {1} else {-1}) * history[j] * pascal_triangle[i][j];
            ans += number;
        }
    }
    println!("{} {}", history.len(), ans);
    ans
}

fn main() {
    let histories = read_input();
    let mut ans: i64 = 0;
    let pascal_triangle = compute_pascal_triangle(30);
    for history in histories {
        ans += predict_prev(&history, &pascal_triangle);
    }
    println!("Answer: {}", ans);
}
