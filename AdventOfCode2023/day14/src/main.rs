use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{prelude::*, BufReader};

fn read_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn get_distance_to_obstacle(rocks: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut d_to_obs = vec![vec![0; rocks[0].len()]; rocks.len()];
    for i in 0..rocks.len() {
        for j in 0..rocks[i].len() {
            if rocks[i][j] == '#' {
                d_to_obs[i][j] = 0;
            } else {
                if i == 0 {
                    d_to_obs[i][j] = 0;
                } else {
                    if rocks[i - 1][j] == '#' {
                        d_to_obs[i][j] = 0;
                    } else {
                        d_to_obs[i][j] = d_to_obs[i - 1][j] + 1;
                    }
                }
            }
        }
    }
    d_to_obs
}

fn get_count_of_rounded_rocks_till_obstacle(rocks: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut cnt_o_rocks_till_obs = vec![vec![0; rocks[0].len()]; rocks.len()];
    for i in 0..rocks.len() {
        for j in 0..rocks[i].len() {
            if rocks[i][j] == '#' {
                cnt_o_rocks_till_obs[i][j] = 0;
            } else {
                if i == 0 {
                    cnt_o_rocks_till_obs[i][j] = 0;
                } else {
                    cnt_o_rocks_till_obs[i][j] = cnt_o_rocks_till_obs[i - 1][j];
                    if rocks[i - 1][j] == '#' {
                        cnt_o_rocks_till_obs[i][j] = 0;
                    } else if rocks[i - 1][j] == 'O' {
                        cnt_o_rocks_till_obs[i][j] += 1;
                    }
                }
            }
        }
    }

    cnt_o_rocks_till_obs
}

fn solve_puzzle_1(rocks: &Vec<Vec<char>>) -> usize {
    let d_to_obs = get_distance_to_obstacle(rocks);
    let cnt_o_rocks_till_obs = get_count_of_rounded_rocks_till_obstacle(rocks);
    let mut ans = 0;
    for i in 0..rocks.len() {
        let factor = rocks.len() - i;
        for j in 0..rocks[0].len() {
            if rocks[i][j] == 'O' {
                let max_up = d_to_obs[i][j] - cnt_o_rocks_till_obs[i][j];
                ans += factor + max_up;
            }
        }
    }

    ans
}

fn get_rounded_rocks_position(rocks: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut rounded_rocks_position = Vec::new();
    for i in 0..rocks.len() {
        for j in 0..rocks[i].len() {
            if rocks[i][j] == 'O' {
                rounded_rocks_position.push((i, j));
            }
        }
    }

    rounded_rocks_position
}

// Rotate the rocks 2d array clockwise
fn rotate(rocks: &mut Vec<Vec<char>>) {
    let mut new_rocks = vec![vec!['.'; rocks.len()]; rocks[0].len()];
    for i in 0..rocks.len() {
        for j in 0..rocks[i].len() {
            new_rocks[j][rocks.len() - 1 - i] = rocks[i][j];
        }
    }

    *rocks = new_rocks;
}

fn roll_north(rocks: &mut Vec<Vec<char>>) {
    let d_to_obs = get_distance_to_obstacle(rocks);
    let cnt_o_rocks_till_obs = get_count_of_rounded_rocks_till_obstacle(rocks);
    let mut new_rocks = vec![vec!['.'; rocks[0].len()]; rocks.len()];
    for i in 0..rocks.len() {
        for j in 0..rocks[0].len() {
            if rocks[i][j] == 'O' {
                let max_up = d_to_obs[i][j] - cnt_o_rocks_till_obs[i][j];
                new_rocks[i][j] = '.';
                new_rocks[i - max_up][j] = 'O';
            } else {
                new_rocks[i][j] = rocks[i][j];
            }
        }
    }
    *rocks = new_rocks;
}

fn get_hash_rocks(rocks: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for row in rocks {
        for &item in row {
            item.hash(&mut hasher);
        }
    }
    hasher.finish()
}

fn get_north_load(rocks: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for i in 0..rocks.len() {
        let factor = rocks.len() - i;
        for j in 0..rocks[i].len() {
            if rocks[i][j] == 'O' {
                load += factor;
            }
        }
    }

    load
}

const ITERATIONS: usize = 1000000000;

fn solve_puzzle_2(rocks: &mut Vec<Vec<char>>) -> usize {
    /*
    0 - north
    1 - west
    2 - south
    3 - east
    */
    let mut configs_to_iteration: HashMap<u64, usize> = HashMap::new();
    let mut iteration_to_load: HashMap<usize, usize> = HashMap::new();
    for i in 0..ITERATIONS {
        println!("Iteration: {}", i);
        // Rolling north then rotating the matrix
        for rotation in 0..4 {
            roll_north(rocks);
            rotate(rocks);
        }
        println!("Rocks: {:?}", rocks);
        let hash_rocks = get_hash_rocks(rocks);
        if configs_to_iteration.contains_key(&hash_rocks) {
            // Cycle detected between iteration number configs_to_iteration[hash_rocks] and i
            println!(
                "Iteration {:?} makes cycle with {:?}",
                i, configs_to_iteration[&hash_rocks]
            );
            let cycle_length = i - configs_to_iteration[&hash_rocks];
            let cycle_iteration =
                configs_to_iteration[&hash_rocks] + (ITERATIONS - i - 1) % cycle_length;
            return iteration_to_load[&cycle_iteration];
        } else {
            let load = get_north_load(rocks);
            println!("Load: {}", load);
            iteration_to_load.insert(i, load);
            configs_to_iteration.insert(hash_rocks, i);
        }
    }

    // No Cycle found which is a bit sad :(
    iteration_to_load[&(ITERATIONS - 1)]
}

fn main() {
    let mut input = read_input();
    //let ans = solve_puzzle_1(&input);
    let ans = solve_puzzle_2(&mut input);
    println!("Ans: {}", ans);
}
