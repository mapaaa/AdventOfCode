use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_input() -> Vec<Vec<Vec<char>>> {
    let file = File::open("input.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<Vec<char>>> = Vec::new();
    let mut layer: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            if !layer.is_empty() {
                grid.push(layer);
                layer = Vec::new();
            }
        } else {
            let chars: Vec<char> = line.chars().collect();
            layer.push(chars);
        }
    }

    if !layer.is_empty() {
        grid.push(layer);
    }

    grid
}

fn is_equal(row1: &Vec<char>, row2: &Vec<char>) -> bool {
    if row1.len() != row2.len() {
        return false;
    }
    for i in 0..row1.len() {
        if row1[i] != row2[i] {
            return false;
        }
    }
    return true;
}

fn off_by_one(row1: &Vec<char>, row2: &Vec<char>) -> bool {
    let mut differences = 0;
    if row1.len() != row2.len() {
        return false;
    }
    for i in 0..row1.len() {
        if row1[i] != row2[i] {
            differences += 1;
        }
    }
    return differences == 1;
}

fn compute_summary(pattern: &Vec<Vec<char>>, should_smudge_correct: bool) -> (i64, i64) {
    let x = get_reflection(pattern, should_smudge_correct.clone());
    let y = get_reflection(&rotate(pattern), should_smudge_correct.clone());
    (x + 1, y + 1)
}

fn get_reflection(pattern: &Vec<Vec<char>>, should_smudge_correct: bool) -> i64 {
    let mut row_reflection = -1;
    let mut max_reflected = 0;
    for i in 0..pattern.len() {
        let mut x = i;
        let mut y = i + 1;
        let mut matched_all = true;
        let mut cnt = 0;
        let mut smudge_corrected = 0;
        while x >= 0 && y < pattern.len() {
            if !is_equal(&pattern[x], &pattern[y]) {
                if should_smudge_correct && smudge_corrected < 1 {
                    if off_by_one(&pattern[x], &pattern[y]) {
                        smudge_corrected += 1;
                        cnt += 1;
                    } else {
                        matched_all = false;
                        break;
                    }
                } else {
                    matched_all = false;
                    break;
                }
            } else {
                cnt += 1;
            }
            if x == 0 {
                break;
            }
            x -= 1;
            y += 1;
        }
        if cnt > max_reflected && matched_all && smudge_corrected == 1 {
            max_reflected = cnt;
            row_reflection = i as i64;
        }
    }
    return row_reflection;
}

fn rotate(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated = vec![vec!['.'; pattern.len()]; pattern[0].len()];
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            rotated[j][pattern.len() - i - 1] = pattern[i][j];
        }
    }
    /*println!("Initial:");
    print_matrix(pattern);
    println!("Rotated");
    print_matrix(&rotated);*/
    rotated
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn main() {
    let input = parse_input();
    let mut ans_rows = 0;
    let mut ans_cols = 0;
    for i in 0..input.len() {
        let (summary_rows, summary_cols) = compute_summary(&input[i], true);
        ans_rows += summary_rows;
        ans_cols += summary_cols;
        println!("Pattern: {i}, Summary: {summary_rows} {summary_cols}");
    }
    println!("Answer {:?}", ans_cols + 100 * ans_rows);
}
