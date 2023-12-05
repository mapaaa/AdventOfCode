use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const DIRECTIONS_X: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
const DIRECTIONS_Y: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];

fn compute_is_neighbour_with_symbol(matrix: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let n: i32 = matrix.len() as i32;
    let m: i32 = matrix[0].len() as i32;
    let mut is_neigh_w_symb: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];
    for i in 0..n {
        for j in 0..m {
            let i_usize: usize = i as usize;
            let j_usize: usize = j as usize;
            if matrix[i_usize][j_usize] == '.' {
                is_neigh_w_symb[i_usize][j_usize] = 0; // nothing
            } else if !matrix[i_usize][j_usize].is_digit(10) {
                is_neigh_w_symb[i_usize][j_usize] = 0; // symbol
            } else {
                is_neigh_w_symb[i_usize][j_usize] = 2; // just a digit
                for d in 0..8 {
                    let x: i32 = i + DIRECTIONS_X[d];
                    let y: i32 = j + DIRECTIONS_Y[d];
                    if x >= 0 && x < n && y >= 0 && y < m {
                        let x = x as usize;
                        let y = y as usize;
                        if !matrix[x][y].is_digit(10) && matrix[x][y] != '.' {
                            // is a symbol!
                            is_neigh_w_symb[i_usize][j_usize] = 3; // digit neigh with symbol!
                            break;
                        }
                    }
                }
            }
        }
    }
    is_neigh_w_symb
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let n = matrix.len();
    let m = matrix[0].len();
    let is_neigh_w_symb: Vec<Vec<i32>> = compute_is_neighbour_with_symbol(&matrix);
    let mut is_valid_number: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];
    let mut cnt_number = 0;
    let mut numbers: HashMap<i32, i32> = HashMap::new();
    println!("{:?}", is_neigh_w_symb);
    let mut sum: i32 = 0;
    for i in 0..n {
        let mut number: i32 = 0;
        let mut is_valid: bool = false;
        let mut start: i32 = -1;
        for j in 0..m {
            match is_neigh_w_symb[i][j] {
                2 => {
                    number = number * 10 + matrix[i][j].to_digit(10).unwrap() as i32;
                    if start == -1 {
                        start = j as i32;
                    }
                }
                3 => {
                    if start == -1 {
                        start = j as i32;
                    }
                    number = number * 10 + matrix[i][j].to_digit(10).unwrap() as i32;
                    is_valid = true;
                }
                _ => {
                    if is_valid {
                        println!("Adding number = {:?}", number);
                        sum += number;
                        cnt_number += 1;
                        for k in (start as usize)..j {
                            is_valid_number[i][k] = cnt_number;
                        }
                        numbers.insert(cnt_number, number);
                    }
                    is_valid = false;
                    number = 0;
                    start = -1;
                }
            }
        }
        if is_valid {
            sum += number;
            cnt_number += 1;
            println!("Adding number = {:?}", number);
            for k in (start as usize)..m - 1 {
                is_valid_number[i][k] = cnt_number;
            }
            numbers.insert(cnt_number, number);
        }
    }
    println!("Sum is {:?}", sum);
    println!("{:?}", is_valid_number);
    let mut ans: i32 = 0;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == '*' {
                println!("Found new *");
                let mut cnt_neighbours: i32 = 0;
                let mut partial_sum: i32 = 1;
                let mut valid_numbers: HashSet<i32> = HashSet::new();

                let i_usize: usize = i as usize;
                let j_usize: usize = j as usize;
                for d in 0..8 {
                    let x: i32 = (i as i32 + DIRECTIONS_X[d]).try_into().unwrap();
                    let y: i32 = (j as i32 + DIRECTIONS_Y[d]).try_into().unwrap();
                    if x >= 0 && x < (n as i32) && y >= 0 && y < (m as i32) {
                        let x = x as usize;
                        let y = y as usize;
                        if is_valid_number[x][y] != 0 {
                            println!("Found: {:?}", is_valid_number[x][y]);
                            valid_numbers.insert(is_valid_number[x][y]);
                        }
                    }
                }

                println!("Found gear with cnt_neigh = {:?}", valid_numbers.len());
                if valid_numbers.len() == 2 {
                    let mut product: i32 = 1;
                    for x in valid_numbers.iter() {
                        product *= numbers[x];
                    }
                    ans = ans + product;
                }
            }
        }
    }
    println!("Gear ans is {:?}", ans); //81742
}
