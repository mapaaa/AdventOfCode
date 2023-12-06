use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_seeds(line: &Vec<String>) -> Vec<i64> {
    let mut seeds_str: Vec<String> = line.clone();
    seeds_str.remove(0);
    seeds_str
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn solve_part_1(input: &Vec<Vec<String>>) -> i64 {
    let mut seeds: Vec<i64> = get_seeds(&input[0]);
    let mut transformation: Vec<i64> = seeds.clone();
    println!("[Part I] Seeds are {:?}", seeds);
    let mut index: usize = 3;
    while index < input.len() {
        if input[index].len() == 0 {
            index += 2;
            seeds = transformation.clone();
            println!("[Part I] Seeds transformed in {:?}", seeds);
        } else {
            let d_start = input[index][0].parse::<i64>().unwrap();
            let s_start = input[index][1].parse::<i64>().unwrap();
            let range = input[index][2].parse::<i64>().unwrap();
            for (i, transformed_seed) in seeds.iter().enumerate() {
                if s_start <= *transformed_seed && *transformed_seed <= s_start + range - 1 {
                    transformation[i] = d_start + (*transformed_seed - s_start);
                }
            }
            index += 1;
        }
    }
    seeds = transformation.clone();
    println!("[Part I] Seeds transformed in {:?}", seeds);
    println!("[Part I] Minimum is {:?}", seeds.iter().min());
    seeds.iter().min().unwrap().clone()
}

fn transorm_to_seeds_tuples(seeds_intervals: Vec<i64>) -> Vec<(i64, i64)> {
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut index = 0;
    while index < seeds_intervals.len() {
        seeds.push((
            seeds_intervals[index],
            seeds_intervals[index] + seeds_intervals[index + 1] - 1,
        ));
        index += 2;
    }
    seeds
}

fn solve_part_2(input: &Vec<Vec<String>>) -> i64 {
    let mut seeds_intervals: Vec<i64> = get_seeds(&input[0]);
    let mut seeds: Vec<(i64, i64)> = transorm_to_seeds_tuples(seeds_intervals);
    println!("[Part II] Seeds are {:?}", seeds);
    let mut transformation: Vec<(i64, i64)> = Vec::<(i64, i64)>::new();
    let mut index: usize = 3;
    while index < input.len() {
        if input[index].len() == 0 {
            seeds.append(&mut transformation);
            println!(
                "[Part II] After all mappings were evaluation, seeds transformed in {:?}",
                seeds
            );
            transformation = Vec::<(i64, i64)>::new();
            index += 2;
        } else {
            let d_start = input[index][0].parse::<i64>().unwrap();
            let s_start = input[index][1].parse::<i64>().unwrap();
            let range = input[index][2].parse::<i64>().unwrap();
            println!(
                "[Part II] {:?} {:?} dest <- {:?} {:?}",
                d_start,
                d_start + range - 1,
                s_start,
                s_start + range - 1
            );
            let mut seeds2: Vec<(i64, i64)> = Vec::<(i64, i64)>::new();
            for (i, (x_ref, y_ref)) in seeds.iter().enumerate() {
                let x = *x_ref;
                let y = *y_ref;
                if s_start <= x && y <= s_start + range - 1 {
                    println!("[Part II] Cazul I pentru {:?} {:?}", x, y);
                    let a = d_start + (x - s_start);
                    let b = d_start + (y - s_start);
                    transformation.push((a, b));
                } else if x <= s_start && y >= s_start {
                    println!("[Part II] Cazul II");
                    if (s_start - 1) >= x {
                        seeds2.push((x, s_start - 1));
                    }
                    let b = d_start + (y - s_start);
                    if y <= s_start + range - 1 {
                        transformation.push((d_start, b));
                    } else {
                        transformation.push((d_start, d_start + range - 1));
                        if s_start + range <= y {
                            seeds2.push((s_start + range, y));
                        }
                    }
                } else if s_start <= x && x <= s_start + range - 1 {
                    println!("[Part II] Cazul III");
                    let a = d_start + (x - s_start);
                    transformation.push((a, d_start + range - 1));
                    if y >= s_start + range {
                        seeds2.push((s_start + range, y));
                    }
                } else {
                    println!("[Part II] Niciun caz");
                    seeds2.push((x, y));
                }
            }
            index += 1;
            seeds = seeds2.clone();
            println!(
                "[Part II] Seeds transformed in {:?} remaining {:?}",
                transformation, seeds
            );
        }
    }
    seeds.append(&mut transformation);
    println!("[Part II] Seeds transformed in {:?}", seeds);
    println!("[Part II] Minimum is {:?}", seeds.iter().map(|x| x.0).min());
    seeds.iter().map(|x| x.0).min().unwrap()
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
    //let minimum_part_1 = solve_part_1(&input);
    let minimum_part_2 = solve_part_2(&input);
}
