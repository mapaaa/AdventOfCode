use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_input(input: &str) -> (String, Vec<i64>) {
    let mut parts = input.split_whitespace();
    let first_part = parts.next().unwrap().to_string();
    let second_part = parts.next().unwrap();
    let numbers: Vec<i64> = second_part
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    (first_part, numbers)
}

fn read_input() -> Vec<(String, Vec<i64>)> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| parse_input(&l.unwrap())).collect()
}

fn valid_group(start: usize, end: usize, spring_condition: &Vec<char>) -> i64 {
    let mut count_dots = 0;
    for i in start..=end {
        if spring_condition[i] == '.' {
            count_dots += 1;
            break;
        }
    }
    if count_dots != 0 {
        return 0;
    }
    return 1;
}

fn no_character(end: usize, ch: char, spring_condition: &Vec<char>) -> i64 {
    let mut count = 0;
    for i in 0..=end {
        if spring_condition[i] == ch {
            count += 1;
            break;
        }
    }
    if count != 0 {
        return 0;
    }
    return 1;
}

fn solve(spring_condition: &Vec<char>, groups: &Vec<i64>) -> i64 {
    // d_i_j_ch = how many possibilities using the first i elements and satisfy the first j groups
    // ending on ch, where ch is either '.' (0) or '#' (1)
    let mut dp = vec![vec![vec![0; 2]; groups.len() + 1]; spring_condition.len()];
    for (i, spring) in spring_condition.iter().enumerate() {
        match spring {
            '.' => dp[i][0][0] = no_character(i, '#', &spring_condition),
            '#' => dp[i][0][1] = no_character(i, '#', &spring_condition),
            '?' => {
                dp[i][0][0] = no_character(i, '#', &spring_condition);
                dp[i][0][1] = no_character(i, '#', &spring_condition);
            }
            _ => {
                panic!("Not valid character");
            }
        }
        for j in 1..groups.len() + 1 {
            let current_group = groups[j - 1];
            let mut characters = Vec::<char>::new();
            match spring {
                '.' => characters.push('.'),
                '#' => characters.push('#'),
                '?' => {
                    characters.push('.');
                    characters.push('#');
                }
                _ => {
                    panic!("Not valid character");
                }
            }
            for ch in characters.iter() {
                match ch {
                    '.' => {
                        if (i as i64) - 1 >= 0 {
                            dp[i][j][0] += dp[i - 1][j][0] + dp[i - 1][j][1];
                        }
                    }
                    '#' => {
                        let x = (i as i64) - current_group + 1;
                        if x - 1 >= 0 {
                            let x = x as usize;
                            dp[i][j][1] += dp[(x - 1) as usize][j - 1][0]
                                * valid_group(x, i, spring_condition);
                        } else if x == 0 && j == 1 {
                            let x = x as usize;
                            dp[i][j][1] = valid_group(x, i, spring_condition);
                        }
                    }
                    _ => {
                        panic!("Not valid character");
                    }
                }
            }
        }
    }
    if spring_condition.last() == Some(&'.') {
        dp[spring_condition.len() - 1][groups.len()][0]
    } else if spring_condition.last() == Some(&'#') {
        dp[spring_condition.len() - 1][groups.len()][1]
    } else {
        dp[spring_condition.len() - 1][groups.len()][0]
            + dp[spring_condition.len() - 1][groups.len()][1]
    }
}

fn solve_all(input: Vec<(String, Vec<i64>)>) -> i64 {
    let mut ans = 0;
    for (spring_condition, groups) in input.iter() {
        let partial_ans = solve(&spring_condition.chars().collect(), groups);
        println!("partial_ans: {}", partial_ans);
        ans += partial_ans;
    }
    ans
}

fn transform_to_input_puzzle_2(input: Vec<(String, Vec<i64>)>) -> Vec<(String, Vec<i64>)> {
    let mut ans = Vec::<(String, Vec<i64>)>::new();
    for (spring_condition, groups) in input.iter() {
        let mut new_spring_condition = String::new();
        let mut new_groups = Vec::<i64>::new();
        for j in 0..4 {
            new_spring_condition.push_str(&spring_condition);
            new_spring_condition.push('?');
            new_groups.extend(groups);
        }
        new_spring_condition.push_str(&spring_condition);
        new_groups.extend(groups);
        ans.push((new_spring_condition, new_groups));
    }
    ans
}

fn main() {
    let input = read_input();
    let input = transform_to_input_puzzle_2(input);
    println!("input: {:?}", input);
    let ans = solve_all(input);
    println!("Answer is {ans}");
}
