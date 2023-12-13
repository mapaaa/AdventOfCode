use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Bound::{Excluded, Included};

fn read_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn get_rows_without_galaxy(star_map: &Vec<Vec<char>>) -> BTreeSet<usize> {
    let mut double_rows = BTreeSet::new();
    for (i, row) in star_map.iter().enumerate() {
        let mut count = 0;
        for col in row.iter() {
            if *col == '#' {
                count += 1;
            }
        }
        if count == 0 {
            double_rows.insert(i);
        }
    }
    double_rows
}

fn get_cols_without_galaxy(star_map: &Vec<Vec<char>>) -> BTreeSet<usize> {
    let mut double_cols = BTreeSet::new();
    let row_len = star_map.len();
    let col_len = star_map[0].len();
    for j in 0..col_len {
        let mut count = 0;
        for i in 0..row_len {
            if star_map[i][j] == '#' {
                count += 1;
            }
        }
        if count == 0 {
            double_cols.insert(j);
        }
    }
    double_cols
}

fn get_galaxies(star_map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (i, row) in star_map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

const GROWTH_FACTOR: i64 = 1000000 - 1;

fn main() {
    let star_map = read_input();
    let double_rows: BTreeSet<usize> = get_rows_without_galaxy(&star_map);
    let double_cols: BTreeSet<usize> = get_cols_without_galaxy(&star_map);
    let galaxies: Vec<(usize, usize)> = get_galaxies(&star_map);
    let mut ans: i64 = 0;
    for (i, g0) in galaxies.iter().enumerate() {
        for g1 in galaxies.iter().skip(i + 1) {
            // rows
            let r0 = min(g0.0, g1.0);
            let r1 = max(g0.0, g1.0);
            let range_of_double_rows = double_rows.range((Included(&r0), Excluded(&r1)));
            ans += ((r1 - r0) as i64) + (range_of_double_rows.count() as i64) * GROWTH_FACTOR;
            // columns
            let c0 = min(g0.1, g1.1);
            let c1 = max(g0.1, g1.1);
            let range_of_double_cols = double_cols.range((Included(&c0), Excluded(&c1)));
            ans += ((c1 - c0) as i64) + (range_of_double_cols.count() as i64) * GROWTH_FACTOR;
        }
    }
    println!("Answer puzzle 1: {}", ans);
}
