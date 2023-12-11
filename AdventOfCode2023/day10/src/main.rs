use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            l.unwrap().chars().collect()
        })
        .collect()
}

fn get_starting_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No starting position found");
}

const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn is_valid_move(from: &char, to: &char, direction: usize) -> bool {
    match direction {
        0 => {
            // west
            if (*to == '-' || *to == 'L' || *to == 'F') && (*from == '-' || *from == 'J' || *from == '7' || *from == 'S') {
                return true;
            } else {
                return false;
            }
        },
        1 => {
            // east
            if (*from == '-' || *from == 'L' || *from == 'F' || *from == 'S') && (*to == '-' || *to == 'J' || *to == '7'){
                return true;
            } else {
                return false;
            }
        },
        2 => {
            // north
            if (*to == '|' || *to == '7' || *to == 'F') && (*from == '|' || *from == 'L' || *from == 'J' || *from == 'S') {
                return true;
            } else {
                return false;
            }
        },
        3 => {
            // south
            if (*to == '|' || *to == 'L' || *to == 'J') && (*from == '|' || *from == '7' || *from == 'F' || *from == 'S') {
                return true;
            } else {
                return false;
            }
        },
        _ => panic!("Invalid direction")
    }
    return false;
}

fn farthest_position(start_x: usize, start_y: usize, map: &Vec<Vec<char>>) -> (i64, Vec<Vec<i64>>, char) {
    let mut distances: Vec<Vec<i64>> = vec![vec![-1; map[0].len()]; map.len()];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut start_directions: Vec<usize> = Vec::new();
    queue.push_back((start_x, start_y));
    distances[start_x][start_y] = 0;
    let mut max_distance = 0;
    while !queue.is_empty() {    
        let (x, y) = queue.pop_front().unwrap();
        for (d, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let new_x = x as i64 + dx;
            let new_y = y as i64 + dy;
            if new_x < 0 || new_y < 0 || new_x >= map.len() as i64 || new_y >= map[0].len() as i64 {
                continue;
            }
            if map[new_x as usize][new_y as usize] == '.' {
                continue;
            }
            if distances[new_x as usize][new_y as usize] != -1 {
                continue;
            }
            if is_valid_move(&map[x][y], &map[new_x as usize][new_y as usize], d.clone()) == true {
                distances[new_x as usize][new_y as usize] = distances[x][y] + 1;
                if distances[new_x as usize][new_y as usize] > max_distance {
                    max_distance = distances[new_x as usize][new_y as usize];
                }
                if x == start_x && y == start_y {
                    start_directions.push(d);
                }
                queue.push_back((new_x as usize, new_y as usize));
            }
        }
    }
    let mut true_character_for_start = ' ';
    match (start_directions[0], start_directions[1]) {
        (0, 1) | (1, 0) => {
            true_character_for_start = '-';
        },
        (0, 2) | (2, 0) => {
            true_character_for_start = 'J';
        },
        (0, 3) | (3, 0) => {
            true_character_for_start = '7';
        },
        (1, 2) | (2, 1) => {
            true_character_for_start = 'L';
        },
        (1, 3) | (3, 1) => {
            true_character_for_start = 'F';
        },
        (2, 3) | (3, 2) => {
            true_character_for_start = '|';
        },
        (_, _) => panic!("Invalid start directions")
    }
    (max_distance, distances, true_character_for_start)
}

fn compute_vertical_crossing_edges(map: &Vec<Vec<char>>, distances: &Vec<Vec<i64>>) -> Vec<Vec<(i64, i64)>> {
    let mut compute_vertical_crossing_edges: Vec<Vec<(i64, i64)>> = vec![vec![(0, 0); map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in (0..map[i].len()).rev() {
            if j + 1 < map[i].len() {
                compute_vertical_crossing_edges[i][j] = compute_vertical_crossing_edges[i][j + 1];
            }
            if distances[i][j] != -1 {
                if map[i][j] == '|' {
                    compute_vertical_crossing_edges[i][j].0 += 1;
                    compute_vertical_crossing_edges[i][j].1 += 1;
                } else if map[i][j] == 'L' {
                    compute_vertical_crossing_edges[i][j].0 += 1; // north
                } else if map[i][j] == 'J' {
                    compute_vertical_crossing_edges[i][j].0 += 1;
                } else if map[i][j] == '7' {
                    compute_vertical_crossing_edges[i][j].1 += 1; // south
                } else if map[i][j] == 'F' {
                    compute_vertical_crossing_edges[i][j].1 += 1;
                }
            }
        }
    }
    compute_vertical_crossing_edges
}

fn main() {
    let map = read_input();
    let (x, y) = get_starting_position(&map);
    println!("Starting position: ({}, {})", x, y);
    let (ans, distances, true_character_for_start) = farthest_position(x.clone(), y.clone(), &map);
    let mut map = map.clone();
    map[x][y] = true_character_for_start;
    println!("Answer puzzle 1: {}", ans);
    println!("distances: {:?}", distances);
    let compute_vertical_crossing_edges = compute_vertical_crossing_edges(&map, &distances);
    println!("compute_vertical_crossing_edges: {:?}", compute_vertical_crossing_edges);
    let mut cnt_interior_tiles = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if distances[i][j] == -1 {
                let mut minim = compute_vertical_crossing_edges[i][j].0;
                if compute_vertical_crossing_edges[i][j].1 < minim {
                    minim = compute_vertical_crossing_edges[i][j].1;
                }
                if minim % 2 == 1 {
                  println!("({}, {})", i, j);  
                  cnt_interior_tiles += 1;
                }
            } 
        }
    }
    println!("Answer puzzle 2: {cnt_interior_tiles}");
}